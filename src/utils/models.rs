use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    pub id: String,
    pub board: String,
    pub thumb_url: String,
    pub content: String,
    pub username: String,
    pub ref_id: String,
    pub time: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserPost {
    pub thumb_url: Option<String>,
    pub content: String,
    pub username: Option<String>,
    pub ref_id: Option<String>,
}