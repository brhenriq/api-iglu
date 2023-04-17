use async_once::AsyncOnce;
use lazy_static::lazy_static;
use sqlx::postgres::PgPoolOptions;
use std::env;

lazy_static! {
    pub static ref DATABASE: AsyncOnce<sqlx::PgPool> = AsyncOnce::new(async {
        let url_connection = env::var("DATABASE_URL").expect("Error on load DATABASE URL env");
        println!("URL {:?}", url_connection);
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&url_connection)
            .await
            .expect("Error on connect to the database")
    });
}

#[tokio::test]
async fn test_connection() {
    dotenv::dotenv().ok();
    let row: (i32,) = sqlx::query_as("SELECT 1")
        .fetch_one(crate::database::DATABASE.get().await)
        .await
        .unwrap();
    println!("ROW: {:?}", row);
    assert!(row == (1,), "Query is working");
}
