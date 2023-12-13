use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;

use password_hash::{SaltString, PasswordHasher};
use argon2::Argon2;

use crate::models;
use models::user::User;

#[post("/login")]
pub async fn login(body: web::Json<User>, pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as::<_, User>(r#"SELECT * FROM "user" WHERE username = $1"#)
        .bind(&body.username)
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(login) => {
            let salt = login.salt.unwrap();
            let salt_string = SaltString::from_b64(&salt).unwrap();
            let password_hash = Argon2::default().hash_password(body.password.as_ref().unwrap().as_bytes(), salt_string.as_salt()).unwrap();

            if login.password == Some(password_hash.to_string()) {
                HttpResponse::Ok().body(format!("Login Successful {}", body.username.as_ref().unwrap()))
            } else {
                HttpResponse::Unauthorized().finish()
            }
        }
        Err(_) => HttpResponse::Unauthorized().finish(),
    }
}