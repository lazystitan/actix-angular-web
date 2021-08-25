use std::env;
use std::hash::{Hash, Hasher};

use diesel::{insert_into, PgConnection, prelude::*, r2d2::ConnectionManager, r2d2::PooledConnection, result::Error, update};
use rand::Rng;

use crate::models::{Post, PostInsert, TokenHistory};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Hash)]
struct TokenProp {
    time: u128,
    number: i32,
}

#[derive(Clone)]
pub struct DataService {
    pool: DbPool,
}

impl DataService {
    pub fn new(stage: &str) -> Self {
        let mut db_url = "DATABASE_URL";
        if stage == "prod" {
            db_url = "DATABASE_URL_PROD";
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
        use crate::schema::posts::dsl::*;
        let connection = &self.conn();
        let results = posts.filter(published.eq(true)).load::<Post>(connection);
        return results;
    }

    pub fn get_post(&self, post_id: i32) -> Result<Post, Error> {
        use crate::schema::posts::dsl::*;
        let connection = &self.conn();
        let result = posts
            .filter(published.eq(true))
            .filter(id.eq(post_id))
            .first::<Post>(connection);
        return result;
    }

    pub fn get_latest_add_post(&self) -> Result<Post, Error> {
        use crate::schema::posts::dsl::*;
        let connection = &self.conn();
        let result = posts.filter(published.eq(true))
            .order(id.desc())
            .first::<Post>(connection);
        return result;
    }

    pub fn delete_post(&self, post_id: i32) -> Result<usize, Error> {
        use crate::schema::posts::dsl::*;
        let connection = &self.conn();
        let result = update(posts).filter(id.eq(post_id))
            .set(published.eq(false))
            .execute(connection);
        return result;
    }

    pub fn update_post(&self, post_id: i32, post: PostInsert) -> Result<usize, Error> {
        use crate::schema::posts::dsl::*;
        let connection = &self.conn();
        let result = update(posts).filter(id.eq(post_id))
            .set((title.eq(post.title), digest.eq(post.digest), content.eq(post.content)))
            .execute(connection);
        return result;
    }

    pub fn add_post(&self, post: PostInsert) -> Result<usize, Error> {
        use crate::schema::posts::dsl::*;
        let connection = &self.conn();
        match insert_into(posts)
            .values(
                (
                    title.eq(post.title),
                    author.eq("riton"),
                    content.eq(post.content),
                    published.eq(true),
                    digest.eq(post.digest)
                )
            ).execute(connection) {
            Ok(rows_inserted) => { Ok(rows_inserted) }
            Err(e) => { Err(e) }
        }
    }

    pub fn gen_token(&self) -> Result<String, Error> {
        use crate::schema::token_history::dsl::*;
        use chrono::prelude::*;
        use chrono::Duration;
        use std::time;
        use std::collections::hash_map;
        let time = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH).unwrap().as_millis();

        let mut rng = rand::thread_rng();
        let number = rng.gen::<i32>();
        let token_prop = TokenProp {
            time,
            number,
        };
        let mut hasher = hash_map::DefaultHasher::new();
        token_prop.hash(&mut hasher);
        let gened_token = hasher.finish().to_string();

        let connection = &self.conn();

        match insert_into(token_history).values(
            (
                token.eq(&gened_token),
                expire_time.eq((Local::now() + Duration::days(30)).naive_local())
            )
        ).execute(connection) {
            Ok(_) => {
                Ok(gened_token)
            }
            Err(e) => {
                Err(e)
            }
        }
    }

    pub fn validate_token(&self, token_to_validate: &str) -> Result<(), Error> {
        use crate::schema::token_history::dsl::*;
        use chrono::prelude::*;
        let connection = &self.conn();
        let now = Local::now().naive_local();
        let result = token_history.filter(token.eq(token_to_validate))
            .filter(expire_time.gt(now))
            .first::<TokenHistory>(connection);
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }
}

#[cfg(test)]
mod db_test {
    use super::*;

    #[test]
    fn posts_test() {
        dotenv::dotenv().ok();
        let pool = DataService::new("dev");
        if let Ok(effect_rows) = pool.add_post(PostInsert {
            title: "test_title".to_string(),
            content: "test_content".to_string(),
            digest: "test_digest".to_string(),
        }) {
            assert_eq!(effect_rows, 1);
            if let Ok(latest_post) = pool.get_latest_add_post() {
                assert_eq!(latest_post.title, "test_title");
                assert_eq!(latest_post.content, "test_content");
                assert_eq!(latest_post.digest, "test_digest");
                if let Ok(effect_rows) = pool.delete_post(latest_post.id) {
                    assert_eq!(effect_rows, 1);
                    return;
                }
            }
        }
        assert!(false);
    }

    #[test]
    fn token_test() {
        dotenv::dotenv().ok();
        let pool = DataService::new("dev");
        if let Ok(gened_token) = pool.gen_token() {
            assert_eq!(pool.validate_token(&gened_token), Ok(()));
            assert_ne!(pool.validate_token("123123"), Ok(()));
            return;
        }
        assert!(false);
    }
}