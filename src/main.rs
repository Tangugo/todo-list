mod model;
mod dao;
mod route;
mod services;

// use env_logger;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use sea_orm::Database;
use actix_web::web::Data;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // env_logger::init();
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // let db = Database::connect(database_url).await.expect("Failed to connect to database");
    let mut opt = sea_orm::ConnectOptions::new(database_url.clone());
    opt.max_connections(200);
    let db = Database::connect(opt).await.expect("Failed to connect to database");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db.clone()))
            .configure(route::init)
    })
    .workers(8)
    .bind("127.0.0.1:8080")?
    .run()
    .await
}