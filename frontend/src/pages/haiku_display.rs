use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
//use wasm_bindgen::{JsCast, UnwrapThrowExt};
//use web_sys::{HtmlInputElement, InputEvent};
use gloo_net::http::Request;

use common::{haiku::*};
use crate::pages::haiku_svg::HaikuDraw;

const BACKEND: &str = "http://localhost:9000";

#[derive(Properties, PartialEq)]
pub struct Props {}

pub struct HaikuDisplay {
    picked_haiku: Haiku,
    kanjis_list: Vec<Kanji>,
//    reveal_line: Vec::<i32>,
//    reveal_line_romaji: Vec::<i32>,
//    reveal_line_meaning: Vec::<i32>,
    is_hovered: bool,
    hovered_text: Kanji,
    playit: bool,
}
pub enum Msg {
    PickedHaiku(Haiku, Vec<Kanji>),
//    RevealHiragana(i32),
//    RevealRomaji(i32),
//    RevealMeaning(i32),
//    RevealAll(i32),
    MouseOver(Kanji),
    DisplayAnimation(),
}

impl Component for HaikuDisplay {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self::fetch_haiku(ctx);
        Self {
            picked_haiku: Haiku::default(),
            kanjis_list: vec![],
//            reveal_line: vec![],
//            reveal_line_romaji: vec![],
//            reveal_line_meaning: vec![],
            is_hovered: false,
            hovered_text: Kanji::default(),
            playit: true,
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::PickedHaiku(hai, kl) => {
                self.picked_haiku = hai;
                self.kanjis_list = kl;
                true
            }
            /*
            Msg::RevealHiragana(idx) => {
                if ! self.reveal_line.contains(&idx) { self.reveal_line.push(idx); }
                else {  self.reveal_line.retain(|value| *value != idx); }
                //self.reveal_line = idx;
                true
            }
            Msg::RevealRomaji(idx) => {
                if ! self.reveal_line_romaji.contains(&idx) { self.reveal_line_romaji.push(idx); }
                else {  self.reveal_line_romaji.retain(|value| *value != idx); }
                true
            }
            Msg::RevealMeaning(idx) => {
                if ! self.reveal_line_meaning.contains(&idx) { self.reveal_line_meaning.push(idx); }
                else {  self.reveal_line_meaning.retain(|value| *value != idx); }
                true
            }
            Msg::RevealAll(idx) => {
                if ! self.reveal_line.contains(&idx) { self.reveal_line.push(idx); }
                else {  self.reveal_line.retain(|value| *value != idx); }
                if ! self.reveal_line_romaji.contains(&idx) { self.reveal_line_romaji.push(idx); }
                else {  self.reveal_line_romaji.retain(|value| *value != idx); }
                if ! self.reveal_line_meaning.contains(&idx) { self.reveal_line_meaning.push(idx); }
                else {  self.reveal_line_meaning.retain(|value| *value != idx); }
                true
            }
            */
            Msg::MouseOver(txt) => {
                //if self.is_hovered { self.is_hovered = false }
                //else {
                    self.is_hovered = true;
                //}
                self.hovered_text = txt;
                true
            }
            Msg::DisplayAnimation() => {
                if self.playit { self.playit = false; }
                else { self.playit = true; }
                true
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let _link = ctx.link();

        let mut playit_visibility = "overlay_all_hidden";
        let mut card_visibility = "card_ok";
        if self.playit {
            playit_visibility = "overlay_all";
            card_visibility = "card_all_hidden";
        }
        else {
            playit_visibility = "overlay_all_hidden";
            card_visibility = "card_ok";
        }

        //let mut display_style = "display:none";

        let my_haiku = self.picked_haiku.clone();
        let my_kanjis = self.kanjis_list.clone();
        let my_kanji = self.hovered_text.clone();

        //let reveal_hiragana: Vec::<i32> = self.reveal_line.clone();
        //let reveal_romaji: Vec::<i32> = self.reveal_line_romaji.clone();
        //let reveal_line_meaning: Vec::<i32> = self.reveal_line_meaning.clone();

        html!{
            <>
            //<nav class="nav has-shadow">
                <div class="nav-left-middle buttons are-medium">
                    <button class="button"
                    onclick={
                    ctx.link().callback(move |_| Msg::DisplayAnimation())
                    }>{ "🞂" }</button>
                    </div>
            //</nav>

            <div class="container">

            <div id={ format!("{}",  playit_visibility) }>
                 <HaikuDraw lines={ my_haiku.haiku_line.clone() } />
            //   <img src="./resources/haiku/basho.svg" alt={ my_haiku.title } width="600" height="800" />
            </div>


            <div id={card_visibility} class="column is-6" >
            <div class="card">

             <header class="card-header-title">
                <p class="title is-6">
                { my_haiku.author }{ ", " }{ my_haiku.created }
                </p>
             </header>

             <div class="card-content" >
             <div class="haiku_container">
             for l in my_haiku.haiku_line.clone() {
                <span class={format!("haiku_column {} has-text-light-95", l.image)}>
                    { for l.line.chars().map(|ch| {
                        let kanji = my_kanjis.iter().find(|k| k.char == ch);

                        match kanji {
                            Some(k) => {
                                let k = k.clone();
                                html! {
                                    <span class="haiku_pointing" onmouseover={ctx.link().callback(move |_| Msg::MouseOver(k.clone()))}>
                                        { ch }
                                    </span>
                                }
                            },
                            None => html! { <span>{ ch }</span> }
                        }
                    })}
                </span>// haiku_column
             }
             </div> //haiku_container
             </div> //card content

             <footer class="card-footer">
             <div class="footer_container_kanji_detail">
             <div class="footer_kanji_detail is-size-7">

                for l in my_haiku.haiku_line {
                    //if reveal_hiragana.contains(&l.order) {
                        <span >
                        { l.reading }<br />

                        { l.romaji }<br />

                        { l.meaning }<br />
                        </span>
                }
                </div></div>

                <br />
                <div class="dek">{ my_haiku.deck }</div>

             </footer>
           </div>
           </div>

           if self.is_hovered {
               <span class="kanji_detail">
                    <h1>{ my_kanji.char }</h1>
                    { "Meaning: " }{ my_kanji.meaning }<br />
                    { "Strokes: " }{ my_kanji.strokes }<br />
                    { "Radical: "}{ my_kanji.radical }<br />
                    <a href={format!("https://jisho.org/search/{}%20%23kanji", my_kanji.char)} target="_blank">{"> Jisho"}</a>
               </span>
           }
           </div>
           </>
        }
    }
}
impl HaikuDisplay {
    fn fetch_haiku(ctx: &Context<Self>) {
        let link = ctx.link().clone();

        spawn_local(async move{
            let fetched_haiku: Haiku = Request::get(format!("{}/haiku", BACKEND).as_str())
                .header("Content-Type", "application/json")
                .send()
                .await
                .unwrap()
                .json()
                .await
                .expect("Failed to parse JSON");
            //link.send_message(Msg::PickedHaiku(fetched_haiku));

            let haiku_id = fetched_haiku.id;
            let kanjis_list: Vec<Kanji> = Request::get(format!("{}/kanjis/{}", BACKEND, haiku_id).as_str())
                .header("Content-Type", "application/json")
                .send()
                .await
                .unwrap()
                .json()
                .await
                .expect("Failed to parse JSON");

            link.send_message(Msg::PickedHaiku(fetched_haiku, kanjis_list));

        });
    }
}
