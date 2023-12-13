use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;
use chrono::Utc;
use base64::{Engine, engine::general_purpose};

use crate::models;
use models::post::Post;

#[post("/posts")]
pub async fn posts_create(body: web::Json<Post>, pool: web::Data<PgPool>) -> impl Responder {
    let time = Utc::now().timestamp();
    
    let random_bytes: Vec<u8> = (0..12).map(|_| rand::random::<u8>()).collect();
    let url_id = general_purpose::URL_SAFE_NO_PAD.encode(&random_bytes);

    let result = sqlx::query(r#"INSERT INTO "post" (url_id, user_id, source, date, caption, media_path, media_url) VALUES ($1, $2, $3, $4, $5, $6, $7)"#)
        .bind(url_id.clone())
        .bind(&body.user_id)
        .bind(&body.source)
        .bind(time)
        .bind(&body.caption)
        .bind(&body.media_path)
        .bind(&body.media_url)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => {
            HttpResponse::Ok().body(format!("Post created"))
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
