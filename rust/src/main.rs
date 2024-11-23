mod entities;
mod filter;
mod dto;

use std::collections::HashMap;
use dotenvy::dotenv;
use serde::Serialize;
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{post, get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use sea_orm::{Database, DatabaseConnection, EntityTrait, FromQueryResult, ColumnTrait, QueryFilter};
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

impl dto::WithTotalTrait for EntityResultRow {
    fn total(&self) -> i64 {
        self.total
    }
}

#[post("/api/list")]
async fn list(query: web::Json<dto::GridFilter>, data: web::Data<AppState>) -> impl Responder {
    let conn = &data.conn;

    let entity_global_searchable = vec![
        entity::Column::Colonne1,
        entity::Column::Colonne2
    ];

    let entity_column_by_name = HashMap::from([
        ("colonne_1".to_string(), entity::Column::Colonne1),
        ("colonne_2".to_string(), entity::Column::Colonne2),
    ]);

    match get_entities_with_total::<Entity, EntityResultRow>(&query, &entity_global_searchable, &entity_column_by_name, &conn).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(err) => {
            println!("{}", err);
            HttpResponse::InternalServerError().body("internal server error")
        },
    }
}

#[get("/api/orm")]
async fn get(data: web::Data<AppState>) -> impl Responder {
    let conn = &data.conn;

    let response = Entity::find()
        .filter(entity::Column::Colonne1.like("abc%"))
        .all(conn)
        .await;

    match response {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(err) => {
            println!("{}", err);
            HttpResponse::InternalServerError().body("internal server error")
        },
    }
}

#[post("/api/login")]
async fn login(req: HttpRequest) -> impl Responder {
    let user_id = req
        .headers()
        .get("Userid")
        .and_then(|header_value| header_value.to_str().ok())
        .unwrap_or("VDL12345");

    let mut response = HashMap::new();
    response.insert("username", user_id);
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("Testing DB connection...");

    let conn = Database::connect(&db_url).await.unwrap();
    let state = AppState { conn };

    println!("OK ! Listening...");

    HttpServer::new(move || {
        let mut cors: Cors;

        match std::env::var("CORS_ORIGIN_ALLOW_ALL") {
            Ok(allow_all) => {
                if allow_all == "True" {
                    cors = Cors::permissive();
                } else {
                    let env_origins = std::env::var("CORS_ALLOWED_ORIGINS").unwrap();
                    let origins: Vec<&str> = env_origins.split(',').collect();

                    cors = Cors::default();

                    for origin in origins {
                        cors = cors.allowed_origin(origin);
                    }
                }
            },
            Err(_) => {
                panic!("CORS_ORIGIN_ALLOW_ALL must be set");
            }
        }

        cors = cors.allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(state.clone()))
            .service(list)
            .service(get)
    })
        .bind(("0.0.0.0", 8000))?
        .run()
        .await
}