use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{HtmlInputElement, InputEvent};
use gloo_net::http::Request;
use itertools::Itertools;
//,word::TranslateRequest
use common::{word::Word,language, language::Language};
use crate::pages::form::Form;

const BACKEND: &str = "http://localhost:9000";

#[derive(Properties, PartialEq)]
pub struct Props {

}
pub struct Dictionary {
    words: Vec<Word>,
    filtered_words: Vec<Word>,
    filter: String,
    selected_serie: Vec<Word>,
    //reload_event: i32,
}

pub enum Msg {
    SetWords(Vec<Word>),
    FilterWords(String),
    SelectSerie(Vec<Word>),
}
impl Component for Dictionary {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self::fetch_words(ctx);
        Self {
            words: vec![],
            filtered_words: vec![],
            filter: "".to_string(),
            selected_serie: vec![],
            //reload_event: 0,
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetWords(mut words) => {
                words.sort_by_key(|w| w.serie);
                self.filtered_words = words.clone();
                self.words = words;
                true
            }
            Msg::FilterWords(filter) => {
                self.filter = filter.clone();
                self.filtered_words = self.words.clone();

                let tmp: Vec<i32> =
                    self.filtered_words.iter()
                    .filter(|w| w.name.to_lowercase().contains(&filter.to_lowercase()) && w.language.iso == "EN")
                    .map(|s| s.serie).collect();

                self.filtered_words = self.filtered_words.iter()
                    .filter(|w| tmp.contains(&w.serie)).cloned()
                     .collect();
                true // Return true to trigger a re-render
            }
            Msg::SelectSerie(ws) => {
                self.selected_serie = ws;
                true
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let oninput = link.callback(|e: InputEvent| {
            let event: Event = e.dyn_into().unwrap_throw();
            let event_target = event.target().unwrap_throw();
            let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
            Msg::FilterWords(target.value())
        });

        let mut lg: Vec<Language> = language::lang();
        lg.sort_by_key(|l| l.iso.clone());
        //let rl = self.reload_event;

        html!{
            <>
            <aside class="menu">
                <table class="table">
                <thead>
                    <tr>
                        <th>{ "..." }</th>
                        <th colspan={(lg.clone().len()-1).to_string()}>
                            <input
                                placeholder="Search"
                                class="input is-normal"
                                oninput={ oninput }
                            />
                        </th>
                    </tr>
                    <tr>
                        <th>{"Serie"}</th>
                        // loop in language //
                        for l in lg.clone() {
                            <th>{ l.name }</th>
                        }
                    </tr>
                </thead>
                <tr>
                </tr>
                <tbody>
                    {
                        for self.filtered_words.iter().chunk_by(|w| w.serie).into_iter().map(|(serie, group)| {
                            // Materialize the group into a Vec inside the map closure
                            let subset: Vec<Word> = group.cloned().collect();
                            let row_words = subset.clone();

                            // Logic for the class (added safety check for empty selected_serie)
                            let is_selected = self.selected_serie.first()
                                .map(|s| s.serie == serie)
                                .unwrap_or(false);

                            html! {
                                <tr
                                    onclick={ctx.link().callback(move |_| Msg::SelectSerie(row_words.clone()))}
                                    class={if is_selected { "is-selected" } else { "" }}
                                >
                                    <td>{ serie }</td>
                                    {
                                        for lg.iter().map(|l| {
                                            let name = subset
                                                .iter()
                                                .find(|x| x.language.iso == l.iso)
                                                .map(|w| w.name.clone())
                                                .unwrap_or_default();
                                            html! { <td>{ name }</td> }
                                        })
                                    }
                                </tr>
                            }
                        })
                    }
                </tbody>
                </table>

            </aside>

        <section class="section">
        <div id="details">
        if self.selected_serie.clone().len() > 0 {// != None {
            <Form set={self.selected_serie.clone()} />
        }
        </div></section>
        </>
        }
    }
}
impl Dictionary {
    fn fetch_words(ctx: &Context<Self>) {
        let link = ctx.link().clone();
        spawn_local(async move {

            let fetched_words: Vec<Word> = Request::get(format!("{}/all", BACKEND).as_str())
                .header("Content-Type", "application/json")
                .send()
                .await
                .unwrap()
                .json()
                .await
                .expect("Failed to parse JSON");

            // Send message back to update component state
            link.send_message(Msg::SetWords(fetched_words));
        });
    }
}
