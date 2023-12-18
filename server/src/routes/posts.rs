use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::models::post::Post;

#[post("/posts")]
async fn posts(body: web::Json<Post>, pool: web::Data<PgPool>) -> impl Responder {
    let mut conditions = Vec::new();

    if let Some(user_id) = &body.user_id {
        conditions.push(format!("user_id = {}", user_id));
    }

    if let Some(post_id) = &body.post_id {
        conditions.push(format!("post_id = {}", post_id));
    }

    if let Some(source) = &body.source {
        conditions.push(format!("source = '{}'", source));
    }

    if let Some(date) = &body.date {
        conditions.push(format!("date = '{}'", date));
    }

    let conditions_joined: String;

    if conditions.is_empty() {
        conditions_joined = "1=1".to_string();
    } else {
        conditions_joined = conditions.join(" AND ");
    };

    let result = sqlx::query_as::<_, Post>(format!(r#"SELECT * FROM "post" WHERE {}"#, conditions_joined).as_str())
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}