mod entities;
mod filter;
mod dto;

use std::collections::HashMap;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use sea_orm::{ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter, FromQueryResult};
use serde::Serialize;
use crate::dto::EntityWithTotal;
use crate::entities::entity;
use crate::entities::entity::Entity;
use crate::filter::{get_entities_with_total};

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
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
async fn list(query: web::Json<dto::FilterQuery>, data: web::Data<AppState>) -> impl Responder {
    let conn = &data.conn;

    let entity_global_searchable = vec![
        entity::Column::Colonne1,
        entity::Column::Colonne2
    ];

    let entity_column_by_name = HashMap::from([
        ("colonne_1".to_string(), entity::Column::Colonne1),
        ("colonne_2".to_string(), entity::Column::Colonne2),
    ]);

    let response = get_entities_with_total::<Entity, EntityResultRow>(&query, &entity_global_searchable, &entity_column_by_name, &conn).await.unwrap();
    HttpResponse::Ok().json(response)
}

#[get("/api/orm")]
async fn get(data: web::Data<AppState>) -> impl Responder {
    let conn = &data.conn;

    let response = Entity::find()
        .filter(entity::Column::Colonne1.like("abc%"))
        .all(conn)
        .await
        .ok();

    println!("{:?}", response);

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
            .service(get)
            .app_data(web::Data::new(state.clone()))
    })
        .bind(("0.0.0.0", 8000))?
        .run()
        .await
}