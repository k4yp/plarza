use reqwest::blocking::Client;
use reqwest::blocking::get;
use serde_xml_rs::from_str;     
use serde_json::{json, Value};
use scraper::{Html, Selector};

pub fn fetch(user_name: &str, count: usize) -> i32{
    let id_url = format!(
        "https://www.youtube.com/@{}",
        user_name
    );
    
    let client = Client::new();

    let id_res = client
        .get(&id_url)
        .send()
        .unwrap();

    println!("{:?}", id_res);
    
    // let id_res_formatted = Html::parse_document(&id_res);

    // let channel_id = id_res_formatted
    //     .select(&Selector::parse("meta[property=\"og:url\"]").unwrap())
    //     .next()
    //     .unwrap()
    //     .value()
    //     .attr("content")
    //     .unwrap()
    //     .rsplit('/')
    //     .next()
    //     .unwrap()
    //     .to_string();

    // let post_url = format!(
    //     "https://www.youtube.com/feeds/videos.xml?channel_id={}",
    //     channel_id
    // );

    // let post_res = client
    //     .get(&post_url)
    //     .send()
    //     .unwrap()
    //     .json::<Value>()
    //     .unwrap();

    // let post_res_formatted = from_str(&post_res).unwrap();

    // let mut results = Vec::new();
    2

    // for entry in &post_res_formatted.feed.entry[..count] {
    //     let date = &entry.published;

    //     let caption = &entry.title;

    //     let media = &entry.media_group.media_thumbnail.url;

    //     let link = &entry.link.href;

    //     let result = json!({
    //         "user": user_name,
    //         "source": "youtube",
    //         "date": date,
    //         "caption": caption.replace("'", "''"),
    //         "media": media,
    //         "link": link,
    //     });

    //     results.push(result);
    // }

    // results
}
