use std::collections::HashMap;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Debug)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
}

impl<T> PaginatedResponse<T> {
    pub(crate) fn new(data: Vec<T>, total: i64) -> Self {
        PaginatedResponse { data, total }
    }
}