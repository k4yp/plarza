use serde::{Serialize, Deserialize};

#[derive(Serialize, Debug, Deserialize, sqlx::FromRow)]
pub struct Post {
    pub post_id: Option<i32>,
    pub user_id: i32,
    pub source: Option<String>,
    pub date: Option<i64>,
    pub caption: Option<String>,
    pub media_path: Option<String>,
    pub media_url: Option<String>
}