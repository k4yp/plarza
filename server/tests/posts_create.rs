mod common;
use common::*;

use server::routes::posts_create::posts_create;

#[actix_rt::test]
async fn test_posts_route() {
    let pool = db_connect().await;

    let app = actix_test::start(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(posts_create)
    });

    let payload = json!({
        "user_id": 1,
        "source": "test.com",
        "caption": "test caption",
        "media_path": "test.png",
        "media_url": "489320482390"
    });

    println!("{}", payload);

    let req = app
        .post("/posts")
        .send_json(&payload)
        .await
        .unwrap();

    assert!(req.status().is_success());
}