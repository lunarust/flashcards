use serde::{Deserialize, Serialize};
use serde_json;
//{"iso":"DE", "name": "Deutsch", "flag": "DE", "has_gender":true},
const LANG: &str = r#"[
    {"iso":"EL", "name": "Ελληνικά", "flag":"GR", "has_gender":true},
    {"iso":"EN", "name": "English", "flag":"GB", "has_gender":false},
    {"iso":"FR", "name": "Français", "flag": "FR", "has_gender":true},
    {"iso":"JA", "name": "日本語", "flag": "JP", "has_gender":false}]"#;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Language {
    pub iso: String,
    pub name: String,
    pub flag: String,
    pub has_gender: bool,
}
pub fn lang() -> Vec<Language> {
    let lg: Vec<Language> = serde_json::from_str(LANG)
        .expect("Drama while parsing language");
    lg
}
pub fn get_lang(is: String) -> Language {
    let ret = lang().into_iter().filter(|l| l.iso==is).next();
    match ret {
        Some(ref Language) => { ret.unwrap().clone() }
        None => Language::default()
    }
}
impl Default for Language {
    fn default() -> Self {
        Self {
            iso: "EN".to_string(),
            name: "English".to_string(),
            flag: "GB".to_string(),
            has_gender: false,
        }
    }
}
