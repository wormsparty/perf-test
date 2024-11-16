use std::collections::HashMap;
use serde::Deserialize;
use convert_case::{Case, Casing};
use sqlx::{Postgres, query_builder::QueryBuilder};

#[derive(Deserialize)]
pub struct QuerySort {
    #[serde(rename = "colId")]
    pub col_id: String,
    pub sort: String,
}

#[derive(Deserialize)]
pub struct QueryFilter {
    #[serde(rename = "filterType")]
    pub filter_type: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub filter: String,
}

#[derive(Deserialize)]
pub struct Query {
    pub start: i64,
    pub end: i64,
    pub filter: HashMap<String, QueryFilter>,
    pub sort: Vec<QuerySort>,
    #[serde(rename = "globalSearch")]
    pub global_search: String,
}

pub fn filter_sort_and_page<'a>(table: &'a str, query: &'a Query, global_searchable_fields: &'a [&'a str]) -> Result<QueryBuilder<'a, Postgres>, &'static str> {
    let mut builder = QueryBuilder::new(format!("
        SELECT *, COUNT(*) OVER() AS total
        FROM {}
        WHERE 1 = 1
    ", table));

    // Filter
    for (field, filter) in &query.filter {
        if filter.filter_type != "text" {
            return Err("Unsupported filter type")
        }

        builder.push(" AND ");

        let col = field.to_case(Case::Snake);
        let filter_val;

        if filter.kind == "startsWith" {
            filter_val = format!("{}%", filter.filter);
        } else if filter.kind == "endsWith" {
            filter_val = format!("%{}", filter.filter);
        } else {
            filter_val = filter.filter.clone();
        }

        // These seem to be safe from injection as the builder replaces spaces with underscores
        if filter.kind == "equals" {
            builder.push(format!("{} = ", col)).push_bind(filter_val);
        } else if filter.kind == "notEquals" {
            builder.push(format!("{} <> ", col)).push_bind(filter_val);
        } else if filter.kind == "contains" {
            builder.push("position(").push_bind(filter_val).push(format!(" in {}) > 0", col));
        } else if filter.kind == "notContains" {
            builder.push("position(").push_bind(filter_val).push(format!(" in {}) = 0", col));
        } else if filter.kind == "startsWith" {
            builder.push(format!("{} ilike ", col)).push_bind(filter_val);
        } else if filter.kind == "endsWith" {
            builder.push(format!("{} ilike ", col)).push_bind(filter_val);
        } else if filter.kind == "blank" {
            builder.push(format!("({} <> '') IS NOT TRUE", col));
        } else if filter.kind == "notBlank" {
            builder.push(format!("{} <> ''", col));
        } else {
            return Err("Unsupported type");
        }
    }

    // Global filter
    if !query.global_search.is_empty() {
        builder.push(" AND (1 = 0");
        let search_value = &query.global_search;

        // This seems to be safe from injection as the builder replaces spaces with underscores
        for field in global_searchable_fields {
            builder.push(" OR position(").push_bind(search_value.clone()).push(format!(" in {}) > 0", field));
        }

        builder.push(")");
    }

    // Sort
    if query.sort.len() > 0 {
        let sort = &query.sort[0];
        let dir = sort.sort.to_case(Case::Lower);

        // The column name is safe from injection, but not the direction
        if dir != "asc" && dir != "desc" {
            return Err("Invalid direction");
        }

        builder.push(format!(" ORDER BY {} {}", sort.col_id.to_case(Case::Snake), dir));
    }

    // Page
    builder.push(" OFFSET ").push_bind(query.start)
           .push(" LIMIT ").push_bind(query.end - query.start);

    Ok(builder)
}
