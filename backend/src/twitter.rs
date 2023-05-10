use serde_json::{json, Value};
use reqwest::blocking::Client;

pub fn fetch(user_name: &str, count: usize, token: &str) -> Vec<Value> {
    use std::time::Instant;
    let now = Instant::now();
    let id_url = format!(
        "https://api.twitter.com/graphql/9zwVLJ48lmVUk8u_Gh9DmA/ProfileSpotlightsQuery?variables=%7B%22screen_name%22%3A%22{}%22%7D",
        user_name
    );

    println!("format: {:.2?}", now.elapsed());
    
    let client = Client::new();

    let id_res = client
        .get(&id_url)
        .header("authorization", token)
        .send()
        .unwrap()
        .json::<Value>()
        .unwrap();

    println!("get request: {:.2?}", now.elapsed());

    let user_id = id_res["data"]["user_result_by_screen_name"]["result"]["rest_id"]
        .to_string()
        .trim_matches('"')
        .to_owned();
    
    let post_url = format!(
        "https://api.twitter.com/graphql/73BM9FU1mPITScnhs6iXug/UserTweets?variables=%7B%22userId%22%3A%22{}%22%2C%22count%22%3A{}%2C%22includePromotedContent%22%3Afalse%2C%22withQuickPromoteEligibilityTweetFields%22%3Atrue%2C%22withSuperFollowsUserFields%22%3Atrue%2C%22withDownvotePerspective%22%3Afalse%2C%22withReactionsMetadata%22%3Afalse%2C%22withReactionsPerspective%22%3Afalse%2C%22withSuperFollowsTweetFields%22%3Atrue%2C%22withVoice%22%3Atrue%2C%22withV2Timeline%22%3Atrue%7D&features=%7B%22responsive_web_twitter_blue_verified_badge_is_enabled%22%3Atrue%2C%22responsive_web_graphql_exclude_directive_enabled%22%3Afalse%2C%22verified_phone_label_enabled%22%3Afalse%2C%22responsive_web_graphql_timeline_navigation_enabled%22%3Atrue%2C%22responsive_web_graphql_skip_user_profile_image_extensions_enabled%22%3Afalse%2C%22tweetypie_unmention_optimization_enabled%22%3Atrue%2C%22vibe_api_enabled%22%3Atrue%2C%22responsive_web_edit_tweet_api_enabled%22%3Atrue%2C%22graphql_is_translatable_rweb_tweet_is_translatable_enabled%22%3Atrue%2C%22view_counts_everywhere_api_enabled%22%3Atrue%2C%22longform_notetweets_consumption_enabled%22%3Atrue%2C%22tweet_awards_web_tipping_enabled%22%3Afalse%2C%22freedom_of_speech_not_reach_fetch_enabled%22%3Afalse%2C%22standardized_nudges_misinfo%22%3Atrue%2C%22tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled%22%3Afalse%2C%22interactive_text_enabled%22%3Atrue%2C%22responsive_web_text_conversations_enabled%22%3Afalse%2C%22longform_notetweets_richtext_consumption_enabled%22%3Afalse%2C%22responsive_web_enhance_cards_enabled%22%3Afalse%7D",
        user_id, count + 2
    );

    println!("2nd format {:.2?}", now.elapsed());

    let post_res = client
        .get(&post_url)
        .header("authorization", token)
        .send()
        .unwrap()
        .json::<Value>()
        .unwrap();
    
    println!("2nd get request: {:.2?}", now.elapsed());

    let posts = &post_res["data"]["user"]["result"]["timeline_v2"]["timeline"]["instructions"][1]["entries"];
    
    let mut results = Vec::new();

    for i in 0..count {
        let post = &posts[i]["content"]["itemContent"]["tweet_results"]["result"]["legacy"];
        
        let date = post["created_at"].as_str().unwrap();
        let caption = post["full_text"].as_str().unwrap();
        let media = post["entities"]["media"][0]["media_url_https"].as_str().unwrap_or("None");
        let link_raw = post["conversation_id_str"].as_str().unwrap();
        let link = format!("https://twitter.com/{}/status/{}",user_name, link_raw);
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