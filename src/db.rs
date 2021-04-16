use diesel::prelude::*;
use diesel::PgConnection;
use std::env;
use crate::models::Post;
use crate::schema::posts::dsl::*;
use diesel::result::Error;

fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_posts(connection : &diesel::PgConnection) -> Result<Vec<Post>, Error> {
    let results = posts.filter(published.eq(true))
        .load::<Post>(connection);
        // .expect("Error loading posts");
    return results;
}

pub fn get_post(post_id : i32, connection : &diesel::PgConnection) -> Result<Post, Error> {
    let result = posts.filter(published.eq(true))
        .filter(id.eq(post_id))
        .first::<Post>(connection);
    return result;
}

