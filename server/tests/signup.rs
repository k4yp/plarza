mod common;
use common::*;

use server::routes::signup::signup;

#[actix_rt::test]
async fn test_signup_route() {
    let random_bytes: Vec<u8> = (0..12).map(|_| rand::random::<u8>()).collect();
    let username = format!("test_{}",general_purpose::URL_SAFE_NO_PAD.encode(&random_bytes));
    
    let pool = db_connect().await;

    let app = actix_test::start(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(signup)
    });

    let payload = json!({
        "username": username,
        "email": format!("{}@email.com", username),
        "password": "password"
    });

    let req = app
        .post("/signup")
        .send_json(&payload)
        .await
        .unwrap();

    assert!(req.status().is_success());
}