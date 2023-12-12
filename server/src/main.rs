use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Data;
use serde::Deserialize;
use sqlx::PgPool;
use chrono::Utc;
use dotenv_codegen::dotenv;

#[derive(Deserialize, Debug, sqlx::FromRow)]
struct Signup {
    username: String,
    email: String,
    password: String
}

#[derive(Deserialize, Debug, sqlx::FromRow)]
struct Login {
    username: String,
    password: String
}

#[derive(Deserialize, Debug, sqlx::FromRow)]
struct Update {
    username: String,
    password: String,
    email: String,
    bio: String,
    display: String,
    media: String
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Plarza API Root")
}

#[post("/signup")]
async fn signup(body: web::Json<Signup>, pool: web::Data<PgPool>) -> impl Responder {
    let time = Utc::now().timestamp();

    let result = sqlx::query(r#"INSERT INTO "user" (username, email, password, date) VALUES ($1, $2, $3, $4)"#)
        .bind(&body.username)
        .bind(&body.email)
        .bind(&body.password)
        .bind(time)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body(format!("Welcome {}!", body.username)),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/login")]
async fn login(body: web::Json<Login>, pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as::<_, Login>(r#"SELECT username, password FROM "user" WHERE username = $1 AND password = $2"#)
        .bind(&body.username)
        .bind(&body.password)
        .fetch_optional(pool.get_ref())
        .await;

    println!("{:?}",result);

    match result {
        Ok(Some(user)) => {
            if body.password == user.password {
                HttpResponse::Ok().body(format!("Login Successful {}", body.username))
            } else {
                HttpResponse::Unauthorized().body("Invalid password")
            }
        }
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url = format!("postgres://{}:{}@localhost:5432/{}",
                        dotenv!("POSTGRES_USER"),
                        dotenv!("POSTGRES_PASSWORD"),
                        dotenv!("POSTGRES_DB"));

    let pool = PgPool::connect(&url)
        .await
        .expect("Failed to connect to the database.");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(index)
            .service(signup)
            .service(login)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}