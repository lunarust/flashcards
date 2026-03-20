use serde::{Deserialize, Serialize};
use serde_json;

use reqwest::Client;
use std::time::Duration;
use std::fs;
use std::str;
use serde_json::{Result, Value};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Kanjialive {
    
}
