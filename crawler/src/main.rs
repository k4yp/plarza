use reqwest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

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

    let client = reqwest::Client::new();
    let response = client.post(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(body)
        .send()
        .await?;

    if response.status().is_success() {
        let response_body = response.text().await?;
        
        println!("{}", response_body);
    } else {
        println!("Request failed with status code: {}", response.status());
    }

    Ok(())
}
