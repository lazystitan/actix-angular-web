use crate::models::{Post, PostInsert};
use crate::schema::posts::dsl::*;
use diesel::{prelude::*, r2d2::ConnectionManager, r2d2::PooledConnection, result::Error, PgConnection, insert_into};
use std::env;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct DataService {
    pool : DbPool
}

impl DataService {
    pub fn new(stage : &str) -> Self {
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
        Self { pool }
    }

    fn conn(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.pool.get().expect("couldn't get db connection from pool")
    }

    pub fn get_posts(&self) -> Result<Vec<Post>, Error> {
        let connection = &self.conn();
        let results = posts.filter(published.eq(true)).load::<Post>(connection);
        return results;
    }

    pub fn get_post(&self, post_id: i32) -> Result<Post, Error> {
        let connection = &self.conn();
        let result = posts
            .filter(published.eq(true))
            .filter(id.eq(post_id))
            .first::<Post>(connection);
        return result;
    }

    pub fn add_post(&self, post: PostInsert) -> Result<(), Error> {
        let connection = &self.conn();
        match insert_into(posts)
            .values(
                (
                    title.eq(post.title),
                    author.eq("riton"),
                    content.eq(post.content),
                    published.eq(true)
                )
            ).execute(connection) {
            Ok(_) => {
                Ok(())
            }
            Err(e) => {
                Err(e)
            }
        }
    }

}

#[cfg(test)]
mod db_test {

    use super::*;

    #[test]
    fn insert_test() {
        dotenv::dotenv().ok();
        let pool = DataService::new("dev");
        pool.add_post(PostInsert {
            title: "".to_string(),
            content: "".to_string()
        });
    }
}