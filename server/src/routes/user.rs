use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::models;
use models::user::User;

#[get("/user")]
pub async fn user(body: web::Json<User>, pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as::<_, User>(r#"SELECT * FROM "user" WHERE username = $1"#)
        .bind(&body.username)
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}