//use std::time::Duration;
use warp::{http::StatusCode, Reply};
//use reqwest;

use itertools::Itertools;

use rust_translate::translate;
//use rust_translate::supported_languages::get_languages;

use common::{word::Word, language::Language};
use crate::db;

pub async fn save_words(body: Vec<Word>) -> Result<impl Reply, warp::Rejection> {
    let tmp: Vec<i32> =  body.iter().map(|w| w.serie).sorted().dedup().collect();

    if tmp[0] == 0 {
        let _ = db::upsert_words(body).await;
    }
    else {
        let _ = db::update_words(body).await;

    }
    Ok(StatusCode::OK)
}
pub async fn all() -> Result<impl Reply, warp::Rejection> {
    let res = db::get_all().await.unwrap();
    Ok(warp::reply::json(&res))
}
pub async fn get_random_word() -> Result<impl Reply, warp::Rejection> {
    let list_serie: Vec<Word> = db::get_random_serie().await.unwrap();

    Ok(warp::reply::json(&list_serie))
}
pub async fn trs(body: Vec<Word>) -> Result<impl Reply, warp::Rejection> {
    let ref_language = Language::default().iso;
    let ref_word = body.clone().into_iter().filter(|l| l.language.iso == ref_language).next().unwrap().name;
    let mut res = body.clone();
    for (idx, w) in body.into_iter().enumerate() { //.into_iter().filter(|l| l.language.iso != refLanguage).enumerate() {
        if w.language.iso != ref_language {
        let translated_text =
            translate(&ref_word.clone(), &ref_language, &w.language.iso)
            .await.unwrap();
        res[idx].name = translated_text.clone();
        }
    }

    Ok(warp::reply::json(&res))
}
