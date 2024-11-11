use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn connect_to_db(database_uri: &str) -> PgPool {
    // let database_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL env var should be set");

    let db = PgPoolOptions::new()
        .max_connections(30)
        .connect(database_uri)
        .await
        .expect("Couldn't connect to db");

    db
}
