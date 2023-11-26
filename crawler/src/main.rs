use reqwest;

#[tokio::main]
async fn main() {
    let url = "http://localhost:9515/session";
    
    let body = r#"
        {
            "desiredCapabilities": {
                "browserName": "chrome",
                "goog:chromeOptions": {
                    "args": ["--headless"]
                }
            }
        }
    "#;

    match send_request(url, body).await {
        Ok(response) => println!("Response: {:?}", response),
        Err(err) => eprintln!("Error: {:?}", err),
    }
}

async fn send_request(url: &str, body: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();

    let response = client
        .post(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(body.to_string())
        .send()
        .await?;

    let body = response.text().await?;

    Ok(body)
}
