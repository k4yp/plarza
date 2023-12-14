mod common;
use common::*;

use server::routes::login::login;

#[actix_rt::test]
async fn test_login_route() {
    let pool = db_connect().await;

    let app = actix_test::start(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(login)
    });

    let payload = json!({
        "username": "test",
        "password": "password"
    });

    let req = app
        .post("/login")
        .send_json(&payload)
        .await
        .unwrap();

    assert!(req.status().is_success());
}