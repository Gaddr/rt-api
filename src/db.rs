use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn get_db_pool(db_url: &str) -> Pool<Postgres> {
    let mut attempts: u8 = 0;

    let pool = loop {
        attempts += 1;

        println!("Connecting to DB, attempt {}", attempts);

        match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
        {
            Ok(pool) => {
                println!("Successfully connected to DB!");
                break pool; // Breaks the loop and returns the pool
            }
            Err(_) => {
                if attempts >= 3 {
                    panic!("Could not connect to DB after {} attempts!", attempts);
                } else {
                    continue;
                }
            }
        }
    };

    return pool;
}
