use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Data;
use serde::{Serialize, Deserialize};
use sqlx::PgPool;
use chrono::Utc;
use dotenv_codegen::dotenv;
use base64::{Engine, engine::general_purpose};

#[derive(Deserialize, Debug, sqlx::FromRow)]
struct Signup {
    username: String,
    email: String,
    password: String
}

#[derive(Deserialize, sqlx::FromRow)]
struct Login {
    username: String,
    password: String
}

// #[derive(Serialize, Deserialize, sqlx::FromRow)]
// struct User {
//     username: Option<String>,
//     password: Option<String>,
//     email: Option<String>,
//     bio: Option<String>,
//     display: Option<String>,
//     media: Option<String>
// }

#[derive(Serialize, Debug, Deserialize, sqlx::FromRow)]
struct Post {
    post_id: Option<i32>,
    user_id: i32,
    source: Option<String>,
    date: Option<i64>,
    caption: Option<String>,
    media_path: Option<String>,
    media_link: Option<String>
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

#[post("/posts")]
async fn posts_create(body: web::Json<Post>, pool: web::Data<PgPool>) -> impl Responder {
    let time = Utc::now().timestamp();
    
    let random_bytes: Vec<u8> = (0..12).map(|_| rand::random::<u8>()).collect();
    let url_id = general_purpose::URL_SAFE_NO_PAD.encode(&random_bytes);

    let result = sqlx::query(r#"INSERT INTO "post" (url_id, user_id, source, date, caption, media_path, media_link) VALUES ($1, $2, $3, $4, $5, $6, $7)"#)
        .bind(url_id.clone())
        .bind(&body.user_id)
        .bind(&body.source)
        .bind(time)
        .bind(&body.caption)
        .bind(&body.media_path)
        .bind(&body.media_link)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => {
            HttpResponse::Ok().body(format!("Post created"))
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

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
            .service(posts)
            .service(posts_create)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}