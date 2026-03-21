use mysql::*;
use mysql::prelude::*;

use common::haiku::{Haiku, HaikuLine, Kanji};
//use super::kanjiapi::{Devkanji,*};
use crate::kanjiapi::get_kanji_from_api;


pub async fn insert_haiku(h: Haiku) -> Result<()> {
    let url: String = get_db_url()
        .await
        .expect("DRAMA");
    let pool = Pool::new(url.as_str());
    let mut conn = pool?.get_conn()?;
    println!("{:?}", h);
    // insert haiku
    let res = conn.exec_drop(
        r"INSERT INTO flashcards.haiku
            (author, title, created, deck_id)
            SELECT :author, :title, IFNULL(:created, ''), d.id
            FROM flashcards.haiku_deck d
            WHERE d.name = :deck",
        params! {
            "author" => h.author.clone(),
            "title" => h.title.clone(),
            "created" => h.created.clone(),
            "deck" => h.deck.clone(),
        }
    )?;
    println!("HAIKU: {:?}", res);

    // loop in lines
    let reshl = conn.exec_batch(
        r"INSERT INTO flashcards.haiku_lines
        (haiku_id, line, reading, romaji, meaning, scene, place, image, alt)
        SELECT h.id, :line, :reading, :romaji, :meaning, :scene, :place, :image, :alt
        FROM flashcards.haiku h
        WHERE author = :author AND title = :title;",
        h.haiku_line.iter().map(|hl| params! {
            "line" => hl.line.clone(),
            "reading" => hl.reading.clone(),
            "romaji" => hl.romaji.clone(),
            "meaning" => hl.meaning.clone(),
            "scene" => hl.scene.clone(),
            "place" => hl.order.clone(),
            "image" => hl.image.clone(),
            "author" => h.author.clone(),
            "title" => h.title.clone(),
            "alt" => hl.alt.clone(),
        })
    )?;
    println!("HAIKU Lines: {:?}", reshl);

    // loop in kanji
    let mut kanji_to_insert: Vec<Kanji> = vec![];

    for hl in h.haiku_line {
        // keeping only Kanjis per their Unicode range.
        let kanji_only: String = hl.line.chars()
            .filter(|c| ('\u{4e00}'..='\u{9faf}').contains(c))
            .collect();
        for kan in kanji_only.chars()
        {
            let kanji_exists: bool = check_kanji_exists(kan.to_string()).await.unwrap();
            if !kanji_exists {
                let kd = get_kanji_from_api(kan.to_string()).await;

                kanji_to_insert.push(
                    Kanji {
                        id: 0,
                        char: kd.kanji.chars().next().expect("string is empty"),
                        meaning: kd.heisig_en,
                        strokes: kd.stroke_count,
                        comment: format!("JLPT level {:?}", kd.jlpt.unwrap_or(0)),
                        radical: "".to_string(),
                    }
                );
            }
        }
    }
    let resk = conn.exec_batch("
        INSERT INTO flashcards.kanji
            (kanji, meaning, strokes, comment)
        VALUES(:kanji, :meaning, :strokes, :comment)",
        kanji_to_insert.iter().map(|k| params! {
            "kanji" => k.char.to_string().clone(),
            "meaning" => k.meaning.clone(),
            "strokes" => k.strokes.clone(),
            "comment" => k.comment.clone(),
        })
    );
    println!("Kanji: {:?}", resk);

    Ok(())
}
async fn check_kanji_exists(k: String) -> Result<bool> {
    let url: String = get_db_url()
        .await
        .expect("DRAMA");

    let pool = Pool::new(url.as_str());
    let mut conn = pool.expect("REASON").get_conn()?;

    let select_kanji: Option<i32> = conn
        .query_first(
            format!("SELECT id
            FROM flashcards.kanji WHERE kanji = '{}'", k)
        )?;

    let mut res = true;
    if select_kanji.is_none() { res = false; }
    Ok(res)
}
pub async fn get_current_haiku() -> Haiku {
    let query = "SELECT h.id, h.author, title, IFNULL(assigned, '') as assigned, IFNULL(created, '') as created, d.name as deck
        FROM haiku h
        INNER JOIN haiku_deck d ON (d.id = h.deck_id)
        WHERE assigned IS NOT NULL AND h.assigned < now()";
        // add later a filter on assigned depending on the current date
    run_query_haiku(query.to_string()).await.expect("REASON")
}
pub async fn get_random_haiku() -> Haiku {
    let query = "SELECT h.id, h.author, title, IFNULL(assigned, '') as assigned, IFNULL(created, '') as created, d.name as deck
        FROM haiku h
        INNER JOIN haiku_deck d ON (d.id = h.deck_id)
        ORDER BY RAND( ) LIMIT 1;";
   run_query_haiku(query.to_string()).await.expect("REASON")
}
pub async fn get_kanjis_for_haiku(id: i32) -> Result<Vec<Kanji>> {
    let query = format!("SELECT k.kanji, k.meaning, strokes, comment, IFNULL(radical, '') AS radical, k.id
        FROM kanji k
        JOIN haiku_lines hl ON (line like CONCAT('%',k.kanji,'%'))
        WHERE hl.haiku_id = {}
        ", id);

    let url: String = get_db_url()
        .await
        .expect("DRAMA");

    let pool = Pool::new(url.as_str());
    let mut conn = pool?.get_conn()?;
    let result:Vec<(String, String, i32, String, String, i32)> = conn.query(query)?;
    let mut kanjis_collection: Vec<Kanji> = vec![];
    for k in result {
        kanjis_collection.push(Kanji {
            char: k.0.chars().next().expect("string is empty"),
            meaning: k.1,
            strokes: k.2,
            comment: k.3,
            radical: k.4,
            id: k.5,
        })
    }
    Ok(kanjis_collection)

}
async fn run_query_haiku(query: String) -> Result<Haiku> {
    let url: String = get_db_url()
        .await
        .expect("DRAMA");

    let pool = Pool::new(url.as_str());
    let mut conn = pool?.get_conn()?;
    let result:Vec<(i32, String, String, String, String, String)> = conn.query(query)?;

    let mut h: Haiku = Haiku::default();

    for r in result {
        h.id = r.0;
        h.author = r.1;
        h.title = r.2;
        h.assigned = r.3;
        h.created = r.4;
        h.deck = r.5;
    }
    h.haiku_line = run_query_haiku_line(h.id).await.unwrap();
    Ok(h)
}
async fn run_query_haiku_line(id: i32) -> Result<Vec<HaikuLine>> {
    let url: String = get_db_url()
        .await
        .expect("DRAMA");

    let pool = Pool::new(url.as_str());
    let mut conn = pool?.get_conn()?;
    let query_line = format!("SELECT line, reading, romaji, meaning, scene, place, image, IFNULL(alt, '') as alt
    FROM haiku_lines
    WHERE haiku_id = {}",id);

    let result_line:Vec<(String, String, String, String, String, i32, String, String)> = conn.query(query_line)?;
    let mut hls: Vec<HaikuLine> = vec![];
    for rl in result_line {
        hls.push(
            HaikuLine {
                line: rl.0,
                reading: rl.1,
                romaji: rl.2,
                meaning: rl.3,
                scene: rl.4,
                order: rl.5,
                image: rl.6,
                alt: rl.7,
            }
        )
    }
    //    h.haiku_line = hls;
    Ok(hls)

}
pub async fn get_all_haikus() -> Vec<Haiku> {
    let query = "SELECT h.id, h.author, title, IFNULL(assigned, '') as assigned, IFNULL(created, '') as created, d.name as deck
        FROM haiku h
        INNER JOIN haiku_deck d ON (d.id = h.deck_id)
        ORDER BY h.id;";
    let url: String = get_db_url()
        .await
        .expect("DRAMA");

    let pool = Pool::new(url.as_str());
    let mut conn = pool.expect("REASON").get_conn();
    let result:Vec<(i32, String, String, String, String, String)> = conn.expect("REASON").query(query).expect("REASON");
    let mut h: Vec<Haiku> = vec![];

    for r in result {
        let hl: Vec<HaikuLine> = run_query_haiku_line(r.0).await.expect("REASON");

        h.push(
            Haiku {
            id: r.0,
            author: r.1,
            title: r.2,
            assigned: r.3,
            created: r.4,
            deck: r.5,
            haiku_line: hl,}
        )
    }
    h
}
async fn get_total_haiku() -> i32 {
    let url: String = get_db_url()
        .await
        .expect("DRAMA");

    let pool = Pool::new(url.as_str());
    let conn = pool.expect("Connection??").get_conn();

    let query = "SELECT count(*) FROM haiku;";
    let res = conn.expect("REASON").query_first(query);
    res.unwrap().expect("REASON")
}
async fn get_db_url() -> Result<String, Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok(); // Loads .env file
    Ok(std::env::var("DATABASE_URL")?)
}
