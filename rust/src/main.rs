mod entities;
mod filter;

use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use sea_orm::{Database, DatabaseConnection, DbErr, FromQueryResult};
use serde::Serialize;
use crate::entities::entity;
use crate::entities::entity::Entity;
use crate::filter::{get_entities_with_total, EntityWithTotal};

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[derive(Serialize, Debug)]
struct PaginatedResponse<T> {
    data: Vec<T>,
    total: i64,
}

impl<T> PaginatedResponse<T> {
    fn new(data: Vec<T>, total: i64) -> Self {
        PaginatedResponse { data, total }
    }
}

#[derive(Debug, FromQueryResult, Serialize)]
pub struct EntityResultRow {
    pub id: i32,
    pub colonne_1: String,
    pub colonne_2: String,
    #[serde(skip_serializing)]
    pub total: i64,
}

impl EntityWithTotal for EntityResultRow {
    fn total(&self) -> i64 {
        self.total
    }
}

#[post("/api/list")]
async fn list(query: web::Json<filter::FilterQuery>, data: web::Data<AppState>) -> impl Responder {
    let conn = &data.conn;

    let entity_global_searchable = vec![
        entity::Column::Colonne1,
        entity::Column::Colonne2
    ];

    pub fn entity_get_column_by_name(column_name: &str) -> Result<entity::Column, DbErr> {
        match column_name {
            "colonne_1" => Ok(entity::Column::Colonne1),
            "colonne_2" => Ok(entity::Column::Colonne2),
            _ => Err(DbErr::Custom("Column name not recognized".to_string())),
        }
    }

    let result = get_entities_with_total::<Entity, EntityResultRow>(&query, &entity_global_searchable, entity_get_column_by_name, &conn).await.unwrap();
    let total = result.first().map(|row| row.total).unwrap_or(0);

    let response = PaginatedResponse::new(result, total);

    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = Database::connect(&db_url).await.unwrap();
    let state = AppState { conn };

    println!("Listening...");

    HttpServer::new(move || {
        App::new()
            .service(list)
            .app_data(web::Data::new(state.clone()))
    })
        .bind(("0.0.0.0", 8000))?
        .run()
        .await
}