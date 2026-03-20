use serde::{Deserialize, Serialize};
//use serde_json;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Haiku {
    pub id: i32,
    pub author: String,
    pub title: String,
    pub assigned: String,
    pub created: String,
    pub haiku_line: Vec<HaikuLine>,
    pub deck: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct HaikuLine {
    pub line: String,
    pub reading: String,
    pub romaji: String,
    pub meaning: String,
    pub scene: String,
    pub order: i32,
    pub image: String,
    pub alt: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Kanji {
    pub id: i32,
    pub char: char,
    pub meaning: String,
    pub strokes: i32,
    pub comment: String,
    pub radical: String,
    //pub kanji_meaning: Vec<KanjiMeaning>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct KanjiMeaning {
    pub id: i32,
    pub hiragana: String,
    pub romaji: String,
}


impl Default for Haiku {
    fn default() -> Self {
        Self {
            id: 0,
            author: "".to_string(),
            title: "".to_string(),
            assigned: "".to_string(),
            created: "".to_string(),
            haiku_line: vec![HaikuLine::default()],
            deck: "".to_string(),
        }
    }
}
impl Default for HaikuLine {
    fn default() -> Self {
        Self {
            line: "".to_string(),
            reading: "".to_string(),
            romaji: "".to_string(),
            meaning: "".to_string(),
            scene: "".to_string(),
            order: 0,
            image: "".to_string(),
            alt: "".to_string(),
        }
    }
}

impl Default for Kanji {
    fn default() -> Self {
        Self {
            id: 0,
            char: ' ',
            meaning: "".to_string(),
            strokes: 0,
            comment: "".to_string(),
            radical: "".to_string(),
        }
    }
}
