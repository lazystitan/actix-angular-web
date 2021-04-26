use crate::models::Post;
use crate::schema::posts::dsl::*;
use diesel::{prelude::*, r2d2::ConnectionManager, result::Error, PgConnection};
use std::env;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn build_db_conn_pool(stage : &str) -> DbPool {
    let mut db_url = "DATABASE_URL_DEV";
    if stage == "prod" {
        db_url = "DATABASE_URL";
    }
    let database_url = env::var(db_url).expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .unwrap_or_else(|e|
            panic!("Failed to create pool {:?}", e)
        );
    pool
}

pub fn get_posts(connection: &diesel::PgConnection) -> Result<Vec<Post>, Error> {
    let results = posts.filter(published.eq(true)).load::<Post>(connection);
    // .expect("Error loading posts");
    return results;
}

pub fn get_post(post_id: i32, connection: &diesel::PgConnection) -> Result<Post, Error> {
    let result = posts
        .filter(published.eq(true))
        .filter(id.eq(post_id))
        .first::<Post>(connection);
    return result;
}
