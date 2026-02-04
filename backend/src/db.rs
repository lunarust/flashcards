use mysql::*;
use mysql::prelude::*;
use rand::Rng;
use common::{word::Word};

//const URL: &str = "mysql://myroot:mypotato@localhost:3306/flashcards";

pub async fn update_words(words: Vec<Word>) -> Result<(), Box<dyn std::error::Error>> {
    let url: String = get_db_url()
        .await
        .expect("DRAMA");

    let pool = Pool::new(url.as_str());

    let mut conn = pool?.get_conn()?;
    conn.exec_batch(
        r"UPDATE flashcards.word
            JOIN category c
            LEFT JOIN gender g ON (g.value = :gender)
            SET
            category = c.id,
            gender = g.id,
            name = :name,
            alternate = :alternate,
            definition = :definition
            WHERE lang = :language
            AND serie = :serie
            AND c.value = :category
        ",
        words.iter().filter(|w| w.name != "")
        .map(|w| params! {
                "language" => w.language.iso.clone(),
                "category" => w.category.clone(),
                "gender" => w.gender.clone(),
                "name" => w.name.clone(),
                "alternate" => w.alternate.clone().join(" "),
                "definition" => w.definition.clone(),
                "serie" => w.serie.clone(),
        })
    )?;

    Ok(())
}
pub async fn upsert_words(words: Vec<Word>) -> Result<(), Box<dyn std::error::Error>> {
    let url: String = get_db_url()
        .await
        .expect("DRAMA");

    let pool = Pool::new(url.as_str());

    let serie: i32 = get_serie(words.clone()).await;
    //println!("Serie: {} & WORDS: {:?}", serie, words);

    let mut conn = pool?.get_conn()?;
    conn.exec_batch(
        format!(r"INSERT INTO flashcards.word
        (lang, category, gender, name, alternate, definition, serie)
        SELECT :language, c.id, g.id, :name, :alternate, :definition, {}
        FROM category c
        LEFT JOIN gender g ON (g.value = :gender)
        WHERE c.value = :category
        ON DUPLICATE KEY
        UPDATE gender = g.id,
        alternate = :alternate,
        definition = :definition,
        category = c.id
        ", serie),
        words.iter().filter(|w| w.name != "")
        .map(|w| params! {
                "language" => w.language.iso.clone(),
                "category" => w.category.clone(),
                "gender" => w.gender.clone(),
                "name" => w.name.clone(),
                "alternate" => w.alternate.clone().join(" "),
                "definition" => w.definition.clone(),
        })
    )?;

    Ok(())
}
pub async fn get_random_serie() -> Result<Vec<Word>> {
    let url: String = get_db_url()
        .await
        .expect("DRAMA");

    let pool = Pool::new(url.as_str());
    let mut conn = pool?.get_conn()?;

    let query = "select distinct serie from word";
    let result:Vec<i32> = conn.query(query)?;
    let mut ret: Vec<i32> = vec![];
    for r in result {
        ret.push(r);
    }

    let number = rand::rng().random_range(0..ret.len());
    let ser = get_one(ret[number]).await;

    ser
}

pub async fn get_one(ser: i32) -> Result<Vec<Word>> {
    let url: String = get_db_url()
        .await
        .expect("DRAMA");

    let pool = Pool::new(url.as_str());
    let mut conn = pool?.get_conn()?;

    let query = format!("SELECT w.id, lang, c.value as category, ifnull(g.value, '') as gender, name, alternate, definition, serie
        FROM word w
        LEFT JOIN category c on (c.id = w.category)
        LEFT JOIN gender g on (g.id=w.gender)
        WHERE serie = {}
        ORDER BY serie, lang, name", ser);
    let result:Vec<(i32, String, String, String, String, String, String, i32)> = conn.query(query)?;

    let mut selected_words: Vec<Word> = vec![];
    for r in result {
        let lg = common::language::get_lang(r.1);

        selected_words.push(
            Word {
                id: r.0,
                language: lg,
                category: r.2,
                gender: r.3,
                name: r.4,
                alternate: r.5.split_whitespace().map(str::to_string).collect(),
                definition: r.6,
                serie: r.7 });
    }
    Ok(selected_words)

}
pub async fn get_all() -> Result<Vec<Word>> {

    let url: String = get_db_url()
        .await
        .expect("DRAMA");

    let pool = Pool::new(url.as_str());
    let mut conn = pool?.get_conn()?;

    let query = "SELECT w.id, lang, c.value as category, ifnull(g.value, '') as gender, name, alternate, definition, serie
        FROM word w
        LEFT JOIN category c on (c.id = w.category)
        LEFT JOIN gender g on (g.id=w.gender)
        ORDER BY serie, lang, name";
    let result:Vec<(i32, String, String, String, String, String, String, i32)> = conn.query(query)?;

    let mut selected_words: Vec<Word> = vec![];
    for r in result {
        let lg = common::language::get_lang(r.1);

        selected_words.push(
            Word {
                id: r.0,
                language: lg,
                category: r.2,
                gender: r.3,
                name: r.4,
                alternate: r.5.split_whitespace().map(str::to_string).collect(),
                definition: r.6,
                serie: r.7 });
    }
    Ok(selected_words)
}
async fn get_serie(words: Vec<Word>) -> i32 {

    let url: String = get_db_url()
        .await
        .expect("DRAMA");

    let pool = Pool::new(url.as_str());
    let conn = pool.expect("Connection??").get_conn();

    let mut wh: Vec<String> = [].to_vec();
    for w in words {
        wh.push(format!("(name = '{}' AND lang = '{}')", w.name, w.language.iso));
    }
    let query = format!(r"
        WITH mx AS (select max(serie)+1 AS serie FROM word)
        SELECT ifnull(w.serie, mx.serie) AS serie
        FROM mx
        LEFT JOIN word w
        ON
        ({})
        ", wh.join(" OR "));

        //println!("{}",query);

    let res = conn.expect("REASON").query_first(query);
    res.unwrap().expect("REASON")
}
async fn get_db_url() -> Result<String, Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok(); // Loads .env file
    Ok(std::env::var("DATABASE_URL")?)
}
