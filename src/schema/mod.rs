use diesel::{
	r2d2::{ConnectionManager},
	pg::PgConnection,
	prelude::*,
};
use r2d2::Pool;
use dotenv::dotenv;
use std::env;

// Define a new type alias for a connection pool
pub type PgPool = Pool<ConnectionManager<PgConnection>>;

// Initialize a connection pool from the DATABASE_URL environment variable
pub fn establish_connection_pool() -> PgPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}

// Define your database schema using Diesel's `table!` macro
table! {
    document {
        id -> Int4,
        name -> Varchar,
    }
}
