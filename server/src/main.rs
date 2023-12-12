use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde_json::Value;
use serde::Deserialize;
use sqlx::PgPool;
use chrono::{Utc, SecondsFormat};

#[derive(Deserialize)]
struct User {
    username: String,
    email: String,
    password: String,
    id: Option<u32>
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Plarza Homepage")
}

#[post("/signup")]
async fn signup(body: web::Json<User>, pool: web::Data<PgPool>) -> impl Responder {
    let time = Utc::now().timestamp();

    let result = sqlx::query(r#"INSERT INTO "user" (username, email, password, date) VALUES ($1, $2, $3, $4)"#)
        .bind(&body.username)
        .bind(&body.email)
        .bind(&body.password)
        .bind(time)
        .execute(pool.get_ref())
        .await;
        
    println!("{:?}",result);

    match result {
        Ok(_) => HttpResponse::Ok().body(format!("Welcome {}!", body.username)),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url = "postgres://dev:password@localhost:5432/dev";

    let pool = PgPool::connect(url)
        .await
        .expect("Failed to connect to the database.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(index)
            .service(signup)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}