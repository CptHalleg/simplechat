use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub async fn db_pool() -> Pool<Postgres> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL muss gesetzt sein");

    PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .unwrap()
}
