use warp::{http::StatusCode, Reply};
use itertools::Itertools;

use common::haiku::Haiku;
use crate::db_haiku;


pub async fn fetch_haiku() -> Result<impl Reply, warp::Rejection> {
    let res = db_haiku::get_random_haiku().await;
    //let res = db_haiku::get_current_haiku().await;
    Ok(warp::reply::json(&res))
}

pub async fn fetch_kanji_by_haiku(haiku_id: i32) -> Result<impl Reply, warp::Rejection> {
    let res = db_haiku::get_kanjis_for_haiku(haiku_id).await.unwrap();
    Ok(warp::reply::json(&res))
}

pub async fn save_haiku(body: Haiku) -> Result<impl Reply, warp::Rejection> {
    println!("Haiku handler: Save");
    let res = db_haiku::insert_haiku(body).await.unwrap();
    Ok(StatusCode::OK)
}
