use diesel::prelude::*;
use diesel::PgConnection;
use std::env;
use crate::models::Post;

fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_posts() -> Vec<Post> {
    use crate::schema::posts::dsl::*;
    let connection = establish_connection();
    let results = posts.filter(published.eq(true))
        .load::<Post>(&connection)
        .expect("Error loading posts");
    return results;
}