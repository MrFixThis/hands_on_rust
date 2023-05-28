//! `Pattern searcher` is a application meant to explore the functionality an
//! analize the **Brute force** nature of the [`Boyer-Moore Pattern Searching Algorithm`](
//! https://en.wikipedia.org/wiki/Boyer%E2%80%93Moore_string-search_algorithm)
//! in its *Bad Character* approach.
//!
//! The algorithm's implementation's functionality is exposed through a simple
//! web application using [actix_web].

use actix_files::Files;
use actix_web::{App, HttpServer};

mod handler;
mod matches;
use handler::matches_ctrl;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(matches_ctrl)
            .service(Files::new("/", "static/").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
