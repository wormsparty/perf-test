use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use rust_test::{db,filter};

#[post("/api/list")]
async fn list(query: web::Json<filter::Query>) -> impl Responder {
    let conn = db::Database::new().await;
    let entities_and_count = conn.get_entities(&query);
    HttpResponse::Ok().json(entities_and_count.await.unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(list)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}

