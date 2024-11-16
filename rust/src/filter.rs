use std::collections::HashMap;
use sea_orm::{sea_query::{Alias, Expr, PostgresQueryBuilder, Query}, DatabaseConnection, DbBackend, DbErr, EntityTrait, Iterable, Order, Statement};
use sea_orm::sea_query::extension::postgres::PgExpr;
use crate::dto::{EntityWithTotal, FilterQuery, PaginatedResponse};

pub async fn get_entities_with_total<T: EntityTrait, U: EntityWithTotal>(
    filter: &FilterQuery,
    global_searchable_fields: &Vec<T::Column>,
    column_by_name: &HashMap<String, T::Column>,
    db: &DatabaseConnection,
) -> Result<PaginatedResponse<U>, DbErr> {
    let mut query = Query::select();

    let mut query = query
        .columns(T::Column::iter())
        .expr_as(
            Expr::cust("COUNT(*) OVER()"),
            Alias::new("total"),
        )
        .from(T::default());

    // Filter
    for (name, filter) in &filter.filter {
        let column = Expr::col(*column_by_name.get(name).unwrap());

        match filter.operator.as_str() {
            "equals" => query = query.and_where(column.eq(filter.filter.clone())),
            "notEquals" => query = query.and_where(column.ne(filter.filter.clone())),
            "contains" => query = query.and_where(column.ilike(format!("%{}%", filter.filter))),
            "notContains" => query = query.and_where(column.ilike(format!("%{}%", filter.filter)).not()),
            "startsWith" => query = query.and_where(column.ilike(format!("{}%", filter.filter))),
            "endsWith" => query = query.and_where(column.ilike(format!("%{}", filter.filter))),
            "blank" => query = query.and_where(column.is_null()),
            "notBlank" => query = query.and_where(column.is_not_null()),
            _ => return Err(DbErr::Custom(format!(
                "Opérateur non supporté: {}.",
                filter.operator
            ))),
        }
    }

    // Global filter
    if !filter.global_search.is_empty() {
        let mut or_condition = Expr::col(global_searchable_fields[0]).ilike(format!("%{}%", filter.global_search));

        for field in &global_searchable_fields[1..] {
            or_condition = or_condition.or(Expr::col(*field).ilike(format!("%{}%", filter.global_search)));
        }

        query = query.and_where(or_condition);
    }

    // Sort
    if filter.sort.len() > 0 {
        let sort = &filter.sort.first().unwrap();
        let column = column_by_name.get(&sort.col_id).unwrap();

        query = query.order_by(
            *column,
            if sort.sort.to_lowercase() == "asc" { Order::Asc } else { Order::Desc },
        );
    }

    // Paging
    query = query.offset(filter.start).limit(filter.end - filter.start);

    // Execute query
    let (sql, values) = query.build(PostgresQueryBuilder);

    // For debugging: print the query
    // println!("{}", sql);
    // println!("{:?}", values);

    let rows: Vec<U> = U::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Postgres,
        &sql,
        values,
    ))
        .all(db)
        .await?;

    let total = rows.first().map(|row| row.total()).unwrap_or(0);

    Ok(PaginatedResponse::new(rows, total))
}
