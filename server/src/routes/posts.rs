use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::models;
use models::post::Post;

#[get("/posts")]
async fn posts(body: web::Json<Post>, pool: web::Data<PgPool>) -> impl Responder {
    let result = match body.post_id {
        Some(post_id) => {
            sqlx::query_as::<_, Post>(r#"SELECT * FROM "post" WHERE post_id = $1"#)
                .bind(post_id)
                .fetch_all(pool.get_ref())
                .await
        }
        None => {
            sqlx::query_as::<_, Post>(r#"SELECT * FROM "post""#)
                .fetch_all(pool.get_ref())
                .await
        }
    };

    match result {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}