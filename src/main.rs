use actix_cors::Cors;
use actix_web::{http, web::Data, App, HttpServer};
use dotenv::dotenv;
use reqwest::Client;
use sqlx::{Pool, Postgres};
use std::env;

mod auth;
mod models;
mod services;

mod db;
pub mod queries;

use load_dotenv::load_dotenv;

use crate::services::api::api_scope;

#[derive(Clone)]
pub struct Environment {
    db_url: String,
    secret_key: String,
}

pub struct AppState {
    db: Pool<Postgres>,
    http_client: Client,
    environment: Environment,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    load_dotenv!();

    let environment = Environment {
        db_url: env!("DB_URL").to_string(),
        secret_key: env!("SECRET_KEY").to_string(),
    };

    let pool = db::get_db_pool(&environment.db_url).await;

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(45))
        .http1_only()
        .build()
        .expect("Could not build HTTP client!");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            // .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(Data::new(AppState {
                db: pool.clone(),
                http_client: client.clone(),
                environment: environment.clone(),
            }))
            .service(api_scope())
    })
    .bind(("0.0.0.0", 8084))?
    .run()
    .await
}
