extern crate actix_angular_web;
extern crate diesel;

use self::actix_angular_web::*;
use self::diesel::prelude::*;
use self::models::*;

fn main() {
    use actix_angular_web::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}