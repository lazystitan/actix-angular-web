use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Deserialize, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub content: String,
    pub published: bool,
    pub create_time: NaiveDateTime,
    pub last_update_time: NaiveDateTime,
    pub digest: String,
    pub viewers_number: i32,
    pub like_number: i32,
    pub dislike_number: i32
}

#[derive(Deserialize)]
pub struct LoginFormData {
    pub username: String,
    pub password: String
}


#[derive(Deserialize)]
pub struct PostInsert {
    pub title: String,
    // pub author: String,
    pub content: String,
    // pub published: bool,
    pub digest: String
}

#[derive(Queryable, Deserialize, Serialize)]
pub struct TokenHistory {
    pub id: i32,
    pub token: String,
    pub create_time: NaiveDateTime,
    pub expire_time: NaiveDateTime
}
