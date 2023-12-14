mod common;
use common::*;

use server::routes::posts::posts;

#[actix_rt::test]
async fn test_posts_route() {
    let pool = db_connect().await;

    let app = actix_test::start(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(posts)
    });

    let payload = json!({
        "user_id": 0,
    });

    let req = app
        .get("/posts")
        .send_json(&payload)
        .await
        .unwrap();

    assert!(req.status().is_success());
}