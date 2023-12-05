use reqwest::Client;
use serde_json::Value;
use regex::Regex;
use itertools::Itertools;

const CHROMEDRIVER_URL: &str = "http://localhost:9515";

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

    let session = match request(format!("{CHROMEDRIVER_URL}/session").as_str(), body).await {
        Ok(session) => session,
        Err(err) => err.to_string(),
    };

    let json: Value = serde_json::from_str(&session).unwrap();
    let session_id = json["sessionId"].as_str().unwrap();

    let _ = match request(&format!("{CHROMEDRIVER_URL}/session/{session_id}/url"), r#"{"url": "https://youtube.com/@mrbeast"}"#).await {
        Ok(response) => response,
        Err(err) => err.to_string(),
    };

    let res = match request(&format!("{CHROMEDRIVER_URL}/session/{session_id}/source"), "").await {
        Ok(response) => response,
        Err(err) => err.to_string(),
    };

    let urls = parse_urls(&res, vec![Regex::new(r"watch\?v=([a-zA-Z0-9_-]{11})").unwrap(), Regex::new(r"shorts\?v=([a-zA-Z0-9_-]{11})").unwrap()]);
    println!("{:?}", urls.unwrap())
}

fn parse_urls(res: &str, patterns: Vec<Regex>) -> Option<Vec<&str>>{
    let video_ids: Vec<&str> = patterns[0]
        .captures_iter(res)
        .map(|captures| captures.get(1).unwrap().as_str())
        .collect();

    if video_ids.is_empty() {
        None
    } else {
        let unique_video_ids: Vec<&str> = video_ids.iter().unique().cloned().collect();
        Some(unique_video_ids)
    }
}

async fn request(url: &str, body: &str) -> Result<String, reqwest::Error> {
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