use dotenv_codegen::dotenv;
use sqlx::PgPool;

pub use actix_test;
pub use serde_json::json;
pub use base64::{Engine, engine::general_purpose};
pub use actix_web::App;
pub use actix_web::web::Data;

pub async fn db_connect() -> PgPool {
    let url = format!("postgres://{}:{}@localhost:5432/{}",
                        dotenv!("POSTGRES_USER"),
                        dotenv!("POSTGRES_PASSWORD"),
                        dotenv!("POSTGRES_DB"));

    PgPool::connect(&url)
        .await
        .expect("Failed to connect to the database.")
}