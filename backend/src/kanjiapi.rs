use serde::{Deserialize, Serialize};
use serde_json;

use reqwest::Client;
use std::time::Duration;
use std::fs;
use std::str;
use serde_json::{Result, Value};


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Devkanji {
    pub freq_mainichi_shinbun: Option<i32>,
    pub grade: Option<i32>,
    pub heisig_en: String,
    pub jlpt: Option<i32>,
    pub kanji: String,
    pub kun_readings: Vec<String>,
    pub meanings: Vec<String>,
    pub name_readings: Vec<String>,
    pub notes: Option<Vec<String>>,
    pub on_readings: Vec<String>,
    pub stroke_count: i32,
    pub unicode: String,
}

pub async fn get_kanji_from_api(k: String) -> Devkanji {

        let client = Client::new();

        let dvk: Devkanji = client
            .get(format!("https://kanjiapi.dev/v1/kanji/{}", k))
            .timeout(Duration::from_secs(180))
            .send()
            .await
            .expect("failed to get response")
            .json()
            .await
            .expect("failed to get payload");

    //let val: Value = serde_json::from_str(&doge).unwrap();
    dvk
}
/*
{"freq_mainichi_shinbun":223,
    "grade":1,
    "heisig_en":"water",
    "jlpt":5,
    "kanji":"水",
    "kun_readings":["みず","みず-"],
        "meanings":["water"],
        "name_readings":["うず","ずみ","つ","ど","み","みさ","みつ","みな","みん"],
        "notes":[],
        "on_readings":["スイ"],
        "stroke_count":4,
        "unicode":"6C34"}
*/
