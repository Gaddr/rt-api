use crate::schema::document::dsl::*;
use crate::schema::PgPool;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;

async fn index(pool: web::Data<PgPool>) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let results = document.limit(5).load::<(i32, String)>(&conn).unwrap();
    HttpResponse::Ok().json(results)
}

#[actix_rt::main]
pub async fn main() -> std::io::Result<()> {
    let pool = crate::schema::establish_connection_pool();
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
