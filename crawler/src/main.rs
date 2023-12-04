use reqwest::Client;
use serde_json::Value;

#[tokio::main]
async fn main() {
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

    let session = match send_request("http://localhost:9515/session", body).await {
        Ok(session) => session,
        Err(err) => err.to_string(),
    };

    let json: Value = serde_json::from_str(&session).unwrap();
    let session_id = json["sessionId"].as_str().unwrap();

    let _ = match send_request(&format!("http://localhost:9515/session/{}/url", session_id), r#"{"url": "https://instagram.com/mrbeast"}"#).await {
        Ok(response) => response,
        Err(err) => err.to_string(),
    };

    let res = match send_request(&format!("http://localhost:9515/session/{}/source", session_id), "").await {
        Ok(response) => response,
        Err(err) => err.to_string(),
    };

    println!("{}", res)
}

async fn send_request(url: &str, body: &str) -> Result<String, reqwest::Error> {
    let response = if body != "" {
        Client::new()
            .post(url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(body.to_string())
            .send()
            .await?
    } else {
        Client::new().get(url).send().await?
    };

    let body = response.text().await?;

    Ok(body)
}
