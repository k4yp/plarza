use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;
use chrono::Utc;
use password_hash::{SaltString, PasswordHasher};
use argon2::Argon2;

use crate::models;
use models::user::User;

#[post("/signup")]
async fn signup(body: web::Json<User>, pool: web::Data<PgPool>) -> impl Responder {
    let salt = SaltString::generate(&mut rand::thread_rng());

    let password_hash = Argon2::default().hash_password(body.password.as_ref().unwrap().as_bytes(), salt.as_salt()).unwrap();

    let time = Utc::now().timestamp();

    let result = sqlx::query(r#"INSERT INTO "user" (username, email, password, salt, date) VALUES ($1, $2, $3, $4, $5)"#)
        .bind(&body.username)
        .bind(&body.email)
        .bind(password_hash.to_string())
        .bind(salt.to_string())
        .bind(time)
        .execute(pool.get_ref())
        .await;

    println!("{:?}", result);

    match result {
        Ok(_) => HttpResponse::Ok().body(format!("Welcome {:?}!", body.username)),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}