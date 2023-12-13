use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub bio: Option<String>,
    pub display: Option<String>,
    pub salt: Option<String>,
    pub pfp: Option<String>
}
