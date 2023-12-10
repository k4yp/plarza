use reqwest::Client;
use serde_json::Value;
use regex::Regex;
use itertools::Itertools;
use futures::future;

const CHROMEDRIVER_URL: &str = "http://localhost:9515";

#[tokio::main]
async fn main() {
    // let body = r#"
    //     {
    //         "desiredCapabilities": {
    //             "browserName": "chrome"
    //         }
    //     }
    // "#;

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


    let session2 = match request(format!("{CHROMEDRIVER_URL}/session").as_str(), body).await {
        Ok(session) => session,
        Err(err) => err.to_string(),
    };

    let json: Value = serde_json::from_str(&session2).unwrap();
    let session_id2 = json["sessionId"].as_str().unwrap();

    let session3 = match request(format!("{CHROMEDRIVER_URL}/session").as_str(), body).await {
        Ok(session) => session,
        Err(err) => err.to_string(),
    };

    let json: Value = serde_json::from_str(&session3).unwrap();
    let session_id3 = json["sessionId"].as_str().unwrap();

    let session4 = match request(format!("{CHROMEDRIVER_URL}/session").as_str(), body).await {
        Ok(session) => session,
        Err(err) => err.to_string(),
    };

    let json: Value = serde_json::from_str(&session4).unwrap();
    let session_id4 = json["sessionId"].as_str().unwrap();

    let futures = vec![
                        youtube("nickwhite", session_id),
                        youtube("bawad", session_id2), 
                        youtube("theprimeagen", session_id3),
                        youtube("fireship", session_id4)
                    ];

    let urls = future::join_all(futures).await;

    println!("{:?}", urls);
}

async fn youtube(username: &str, session_id: &str) -> Option<Vec<String>> {
    let _ = match request(&format!("{CHROMEDRIVER_URL}/session/{session_id}/url"), format!("{{\"url\": \"https://youtube.com/@{username}\"}}").as_str()).await {
        Ok(response) => response,
        Err(err) => err.to_string(),
    };

    let res = match request(&format!("{CHROMEDRIVER_URL}/session/{session_id}/source"), "").await {
        Ok(response) => response,
        Err(err) => err.to_string(),
    };

    parse_urls(&res, vec![r"watch\?v=([a-zA-Z0-9_-]{11})".to_string(), r"shorts\?v=([a-zA-Z0-9_-]{11})".to_string()])
}

fn parse_urls(res: &str, patterns: Vec<String>) -> Option<Vec<String>> {
    let video_ids: Vec<String> = Regex::new(&patterns[0])
        .unwrap()
        .captures_iter(res)
        .filter_map(|captures| captures.get(1).map(|m| m.as_str().to_string()))
        .collect();

    if video_ids.is_empty() {
        None
    } else {
        let unique_video_ids: Vec<String> = video_ids.iter().unique().cloned().collect();
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