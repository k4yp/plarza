use reqwest::Client;
use serde_json::Value;
use regex::Regex;
use itertools::Itertools;
use tokio::task;
use std::process::Command;

const CHROMEDRIVER_URL: &str = "http://localhost:9515";

const BODY: &str = r#"
{
    "desiredCapabilities": {
        "browserName": "chrome",
        "goog:chromeOptions": {
            "args": ["--headless", "--disable-gpu", "--blink-settings=imagesEnabled=false"]
        }
    }
}
"#;

#[tokio::main]
async fn main() {
    let mut driver = Command::new("./chromedriver-linux64/chromedriver").spawn().expect("Failed to execute command");
    
    let futures = vec![
        task::spawn(youtube("nickwhite")),
        task::spawn(youtube("bawad")),
        task::spawn(youtube("theprimeagen")),
        task::spawn(youtube("fireship")),
        task::spawn(youtube("t3dotgg")),
        task::spawn(youtube("noboilerplate")),
        task::spawn(youtube("techwithtim")),
        task::spawn(youtube("letsgetrusty")),
        task::spawn(youtube("bigboxswe")),
        task::spawn(youtube("namanhkapur")),
        task::spawn(youtube("fknight")),
        task::spawn(youtube("code_report")),
        task::spawn(youtube("yannickilcher")),
        task::spawn(youtube("coderized")),
        task::spawn(youtube("hyperplexed")),
        task::spawn(youtube("codeaesthetic"))
    ];

    for future in futures {
        let result = future.await.expect("Task panicked");
        println!("{:?}", result.unwrap());
    }

    driver.kill().expect("Failed to kill chromedriver");
}

async fn youtube(username: &str) -> Option<Vec<String>> {
    let session = match request("POST", format!("{CHROMEDRIVER_URL}/session").as_str(), BODY).await {
        Ok(session) => session,
        Err(err) => err.to_string(),
    };

    let json: Value = serde_json::from_str(&session).unwrap();
    let session_id = json["sessionId"].as_str().unwrap().to_string();

    let _ = match request("POST", &format!("{CHROMEDRIVER_URL}/session/{session_id}/url"), format!("{{\"url\": \"https://youtube.com/@{username}\"}}").as_str()).await {
        Ok(response) => response,
        Err(err) => err.to_string(),
    };

    let res = match request("GET", &format!("{CHROMEDRIVER_URL}/session/{session_id}/source"), "").await {
        Ok(response) => response,
        Err(err) => err.to_string(),
    };

    let _ = match request("DELETE", &format!("{CHROMEDRIVER_URL}/session/{session_id}"), "").await {
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

async fn request(method: &str, url: &str, body: &str) -> Result<String, reqwest::Error> {
    let response = match method {
        "POST" =>   Client::new()
                        .post(url)
                        .header(reqwest::header::CONTENT_TYPE, "application/json")
                        .body(body.to_string())
                        .send()
                        .await?,

        "GET" =>    Client::new()
                        .get(url)
                        .send()
                        .await?,

        "DELETE" => Client::new()
                        .delete(url)
                        .send()
                        .await?,
        &_ =>       todo!()
    };

    let body = response.text().await?;

    Ok(body)
}