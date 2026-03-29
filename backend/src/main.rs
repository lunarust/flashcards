extern crate lazy_static;
use warp::{http::Method, Filter};
//use std::fs;

mod handler;
mod db;
mod db_haiku;
mod handler_haiku;
mod generic;
mod kanjiapi;

#[tokio::main]
async fn main() {

    println!("Good day ▼(´ᴥ`)▼ ");

    let all = warp::path!("all");
    let words = warp::path!("save");
    let word_translate = warp::path!("translate");
    let word_random = warp::path!("random");

    let words_routes = words
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handler::save_words)
        .or(word_translate
            .and(warp::post())
            .and(warp::body::json())
            .and_then(handler::trs))
        .or(all
            .and(warp::get())
            .and_then(handler::all))
        .or(word_random
            .and(warp::get())
            .and_then(handler::get_random_word)
        );

    let haiku = warp::path!("haiku");
    let kanjis = warp::path!("kanjis" / i32);
    let haikus = warp::path!("haikus");

    let haiku_routes = haiku
        .and(warp::get())
        .and_then(handler_haiku::fetch_haiku)
        .or(haiku
            .and(warp::post())
            .and(warp::body::json())
            .and_then(handler_haiku::save_haiku))
        .or(kanjis
            .and(warp::get())
            .and_then(handler_haiku::fetch_kanji_by_haiku))
        .or(haikus
            .and(warp::get())
            .and_then(handler_haiku::fetch_all_haikus));

    let routes = words_routes
        .or(haiku_routes)
        .with(
            warp::cors()
            .allow_origin("http://localhost")
            .allow_origin("http://tanit.greece.local:8002/")
            .allow_methods(&[
                Method::OPTIONS,
                Method::GET,
                Method::POST,
                Method::DELETE,
                Method::HEAD,
            ])
            .allow_headers(vec!["allow_origin", "allow_any_origin", "Access-Control-Allow-Origin",
                "Referer", "Control-Request-Headers", "Content-Type"])
            .max_age(300)
            .allow_any_origin(),
    );
    warp::serve(routes).run(([0, 0, 0, 0], 9000)).await;
}
