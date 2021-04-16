use chrono::NaiveDateTime;
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
}
