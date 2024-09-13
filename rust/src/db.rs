use actix_web::{web};
use dotenvy::dotenv;
use rocket::serde::Serialize;
use sqlx::{Row, Pool, Postgres, FromRow, postgres::PgPoolOptions};
use crate::filter;

pub struct Database {
    pub pool: Pool<Postgres>,
}

#[derive(Serialize, FromRow)]
pub struct Entity {
    pub id: i32,
    pub colonne_1: String,
    pub colonne_2: String,
}

#[derive(Serialize)]
pub struct EntityResult {
    pub data: Vec<Entity>,
    pub total: i64,
}

impl Database {
    pub async fn new() -> Self {
        dotenv().ok();

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await.ok().unwrap();

        Database { pool: pool }
    }

    pub async fn get_entities(&self, query: &web::Json<filter::Query>) -> Result<EntityResult, &str> {
        let global_searchable_fields = [
            "colonne_1",
            "colonne_2"
        ];

        let mut builder = filter::filter_sort_and_page("entity", &query, &global_searchable_fields).ok().unwrap();

        let mut result = EntityResult {
            data: Vec::new(),
            total: 0,
        };

        let rows = builder.build()
               .fetch_all(&self.pool)
               .await.unwrap();

        if rows.len() > 0 {
            result.total = rows[0].get("total");

            for row in rows {
                result.data.push(Entity::from_row(&row).unwrap())
            }
        }

        Ok(result)
    }
}
