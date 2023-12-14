mod common;
use common::*;

use server::routes::user::user;

#[actix_rt::test]
async fn test_user_route() {
    let pool = db_connect().await;

    let app = actix_test::start(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(user)
    });

    let payload = json!({
        "username": "test",
    });

    let req = app
        .get("/user")
        .send_json(&payload)
        .await
        .unwrap();

    assert!(req.status().is_success());
}