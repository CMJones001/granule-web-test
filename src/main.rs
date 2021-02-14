mod config;
mod db;
mod handler;
mod models;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use tokio_postgres::NoTls;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up the configuration
    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();
    println!(
        "Starting server at http://{}:{}/",
        config.server.host, config.server.port
    );

    let pool = config.pg.create_pool(NoTls).unwrap();

    // Launch the app
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(handler::status)
            .service(handler::get_experiments)
            .service(handler::get_granules)
    })
    .keep_alive(10)
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}