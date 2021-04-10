use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Queryable, Deserialize, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub content: String,
    pub published: bool,
    pub create_time: NaiveDateTime,
    pub last_update_time: NaiveDateTime,
}