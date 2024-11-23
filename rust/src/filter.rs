use std::collections::HashMap;
use sea_orm::{sea_query::{Alias, Expr, PostgresQueryBuilder, Query}, DatabaseConnection, DbBackend, DbErr, EntityTrait, Iterable, Order, Statement};
use sea_orm::sea_query::extension::postgres::PgExpr;
use sea_orm::sea_query::{Func, SimpleExpr};
use crate::dto::{WithTotalTrait, GridFilter, PaginatedResponse};

macro_rules! unaccent {
    ($col:expr) => {
        SimpleExpr::from(Func::cust(Alias::new("unaccent")).arg($col))
    }
}

pub async fn get_entities_with_total<T: EntityTrait, U: WithTotalTrait>(
    filter: &GridFilter,
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

    // Filter (ignore case)
    for (name, filter) in &filter.filter {
        let column_name = column_by_name.get(name).ok_or(DbErr::Custom("invalid column name".to_string()))?;
        let column = Expr::col(*column_name);
        let filter_no_accent = unidecode::unidecode(&filter.filter);

        match filter.operator.as_str() {
            "equals" => query = query.and_where(unaccent!(column).eq(filter_no_accent)),
            "notEquals" => query = query.and_where(unaccent!(column).ne(filter_no_accent)),
            "contains" => query = query.and_where(unaccent!(column).ilike(format!("%{}%", filter_no_accent))),
            "notContains" => query = query.and_where(unaccent!(column).ilike(format!("%{}%", filter_no_accent)).not()),
            "startsWith" => query = query.and_where(unaccent!(column).ilike(format!("{}%", filter_no_accent))),
            "endsWith" => query = query.and_where(unaccent!(column).ilike(format!("%{}", filter_no_accent))),
            "blank" => query = query.and_where(column.is_null()),
            "notBlank" => query = query.and_where(column.is_not_null()),
            _ => return Err(DbErr::Custom(format!(
                "Unsupported operator: {}.",
                filter.operator
            ))),
        }

    }

    // Global filter (ignore case)
    if !filter.global_search.is_empty() {
        let filter_no_accent = unidecode::unidecode(&filter.global_search);
        let mut or_condition = unaccent!(Expr::col(global_searchable_fields[0])).ilike(format!("%{}%", filter_no_accent));

        for field in &global_searchable_fields[1..] {
            or_condition = or_condition.or(unaccent!(Expr::col(*field)).ilike(format!("%{}%", filter_no_accent)));
        }

        query = query.and_where(or_condition);
    }

    // Sort
    if filter.sort.len() > 0 {
        let sort = &filter.sort.first().unwrap();
        let column = column_by_name.get(&sort.col_id).ok_or(DbErr::Custom("invalid column name".to_string()))?;

        query = query.order_by(
            *column,
            if sort.sort.to_lowercase() == "asc" { Order::Asc } else { Order::Desc },
        );
    } else {
        let column = column_by_name.get("id").ok_or(DbErr::Custom("id column not defined".to_string()))?;
        query = query.order_by(*column,Order::Asc);
    }

    // Paging
    if filter.end != -1 {
        query = query.offset(filter.start as u64).limit((filter.end - filter.start) as u64);
    }

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
