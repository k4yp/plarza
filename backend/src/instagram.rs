use serde_json::{json, Value};
use reqwest;

pub fn fetch(user_name: &str, count: usize, token: &str) -> Vec<Value> {
    let url = format!("https://www.instagram.com/{}/?__a=1&__d=dis", user_name);
    let res = reqwest::blocking::Client::new()
        .get(&url)
        .header("cookie", token)
        .send()
        .unwrap()
        .json::<Value>()
        .unwrap();

    let edges = res["graphql"]["user"]["edge_owner_to_timeline_media"]["edges"].as_array().unwrap();

    let mut results = Vec::new();
    for i in 0..count {
        let edge = &edges[i]["node"];
        let date = edge["taken_at_timestamp"].as_u64().unwrap();
        let caption = edge["edge_media_to_caption"]["edges"][0]["node"]["text"].as_str().unwrap();
        let media = edge["display_url"].as_str().unwrap();
        let link_raw = edge["shortcode"].as_str().unwrap();
        let link = format!("https://www.instagram.com/p/{}/", link_raw);
        let result = json!({
            "user": user_name,
            "source": "instagram",
            "date": date,
            "caption": caption.replace("'", "''"),
            "media": media,
            "link": link
        });
        results.push(result);
    }

    results
}