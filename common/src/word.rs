use serde::{Deserialize, Serialize};
use crate::{language::Language,language};

const WORDTYPE: &str = "nouns verbs adjectives adverbs pronouns prepositions conjunctions interjections";
const GENDER: &str = "masculine feminine neuter";


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TranslateRequest {
    pub word: Word,
    pub to: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Word {
    pub id: i32,
    pub language: Language,
    pub category: String,
    pub gender: String,
    pub name: String,
    pub alternate: Vec<String>,
    pub definition: String,
    pub serie: i32,
}
impl Default for Word {
    fn default() -> Self {
        Self {
            id: 0,
            language: Language::default(),
            category: "nouns".to_string(),
            gender: "".to_string(),
            name: "".to_string(),
            alternate: [].to_vec(),
            definition: "".to_string(),
            serie: 0,
        }
    }
}
pub fn categories() -> Vec<String> {
    let cat: Vec<String> = WORDTYPE.split_whitespace().map(str::to_string).collect();
    cat
}
pub fn get_gender() -> Vec<String> {
    let genders: Vec<String> = GENDER.split_whitespace().map(str::to_string).collect();
    genders
}
pub fn build_default_serie() -> Vec<Word> {
    let lg = language::lang();
    let mut resp: Vec<Word> = vec![];
    for l in lg {
        let mut dd = Word::default();
        dd.language = l;
        resp.push(dd);
    }
    resp
}
