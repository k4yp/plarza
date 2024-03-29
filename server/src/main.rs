use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use actix_web::web::Data;
use dotenv_codegen::dotenv;
use sqlx::PgPool;

mod routes;
mod models;

use routes::signup::signup;
use routes::login::login;
use routes::user::user;
use routes::posts::posts;
use routes::posts_create::posts_create;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Plarza API Root")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let url = format!("postgres://{}:{}@{}:5432/{}",
                        dotenv!("POSTGRES_USER"),
                        dotenv!("POSTGRES_PASSWORD"),
                        dotenv!("POSTGRES_IP"),
                        dotenv!("POSTGRES_DB"));

    let pool = PgPool::connect(&url)
        .await
        .expect("Failed to connect to the database.");

    println!("Connected to the database.");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        
        App::new()
            .wrap(cors)
            .app_data(Data::new(pool.clone()))
            .service(index)
            .service(signup)
            .service(login)
            .service(user)
            .service(posts)
            .service(posts_create)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}