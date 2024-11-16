use std::collections::HashMap;
use sea_orm::{sea_query::{Alias, Expr, PostgresQueryBuilder, Query}, DatabaseConnection, DbBackend, DbErr, EntityTrait, FromQueryResult, Iterable, Order, Statement};
use sea_orm::sea_query::extension::postgres::PgExpr;
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Sort {
    #[serde(rename = "colId")]
    pub col_id: String,
    pub sort: String,
}

#[derive(Deserialize)]
pub struct FieldFilter {
    #[serde(rename = "type")]
    pub operator: String,
    pub filter: String,
}

#[derive(Deserialize)]
pub struct FilterQuery {
    pub start: u64,
    pub end: u64,
    pub filter: HashMap<String, FieldFilter>,
    pub sort: Vec<Sort>,
    #[serde(rename = "globalSearch")]
    pub global_search: String,
}

pub trait EntityWithTotal: FromQueryResult {
    #[allow(dead_code)]
    fn total(&self) -> i64;
}

pub async fn get_entities_with_total<T: EntityTrait, U: EntityWithTotal>(
    filter: &FilterQuery,
    global_searchable_fields: &Vec<T::Column>,
    get_column_by_name_fn: impl Fn(&str) -> Result<T::Column, DbErr>,
    db: &DatabaseConnection,
) -> Result<Vec<U>, DbErr> {
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
        let column = Expr::col(get_column_by_name_fn(name).ok().unwrap());

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
        let column = get_column_by_name_fn(&sort.col_id).ok().unwrap();

        query = query.order_by(
            column,
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

    Ok(rows)
}
