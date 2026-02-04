use yew::prelude::*;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen_futures::spawn_local;
use gloo_net::http::Request;
//use log::info;
use web_sys::HtmlInputElement;
//use itertools::Itertools;

use common::{word::Word,word, language, language::Language};
use crate::pages::subform::SubFormDisplay;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub set: Vec<Word>,
}
#[derive(Properties, PartialEq)]
pub struct Form {
    words: Vec<Word>,
    languages: Vec<Language>,
    label: String,
}
pub enum Msg {
    SaveWord(),
    EditCategory(String),
    EditDefinition(String),
    EditSubForm(Vec<Word>),
    TranslateWord(),
    SetWords(Vec<Word>),
}
impl Component for Form {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let lang = language::lang();
        let mut tmp: Vec<Word> = ctx.props().set.clone();
        // check if we have a word for each language

        let mutmp = tmp.clone();
        let missing_words = lang.iter()
            .filter(|&l| !mutmp.iter().any(|w| w.language == *l))
            .map(|l| Word { language: l.clone(), ..Word::default() });

        tmp.extend(missing_words);

        Self { label: "".to_string(), languages: lang, words: tmp }
    }
    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if ctx.props().set != _old_props.set {
            // check if we have a word for each language

            let mut tmp: Vec<Word> = ctx.props().set.clone();
            let mutmp = tmp.clone();
            let missing_words = self.languages.iter()
                .filter(|&l| !mutmp.iter().any(|w| w.language == *l))
                .map(|l| Word { language: l.clone(), ..Word::default() });

            tmp.extend(missing_words);

            //self.words = ctx.props().set.clone();
            self.words = tmp;
            return true; // Re-render
        }

        false
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::TranslateWord() => {
                let all_words = self.words.clone();
                //info!("TranslateWord {:?}", all_words);
                Self::translate_word(all_words.clone(), ctx);
                //self.words = all_words;
                false
            }
            Msg::SetWords(ws) => {
                self.words = ws.clone();
                //info!("TranslateWord>>>>{:?}", ws);

                true
            }
            Msg::SaveWord() => {
                let all_words = self.words.clone();
                //info!("SaveWord {:?}", all_words);
                Self::save_words(all_words);

                false
            }
            Msg::EditDefinition(c) => {
                let mut all_words = self.words.clone();
                for w in &mut all_words {
                    w.definition = c.clone();
                }
                self.words = all_words;
                false
            }
            Msg::EditCategory(c) => {
                //info!("EditCategory: {:?}", c);
                let mut all_words = self.words.clone();
                for w in &mut all_words {
                    w.category = c.clone();
                }
                self.words = all_words;
                false
            }
            Msg::EditSubForm(w) => {
                self.words = w;
                true
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let selected_set = &ctx.props().set;
        let mut default_category = "nouns";
        if selected_set.len() > 0 { default_category =  &selected_set[0].category; }

        //info!("selected set category: {}", default_category);

        let langs: Vec<Language> = self.languages.clone();
        let cats: Vec<String> = word::categories();

        let edit_definition = link.callback(|e: InputEvent| {
            let event: Event = e.dyn_into().unwrap_throw();
            let event_target = event.target().unwrap_throw();
            let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
            Msg::EditDefinition(target.value())
        });


        html!{
            <>
            <div class="fixed-grid">
                <div class="field is-horizontal">
                    <div class="field-label is-normal"><label class="label">{ "Category:" }</label></div>
                    <div class="field-body"><div class="radios">
                    for c in cats {
                        <label class="radio">
                        <input type="radio" name="category"
                            checked={
                                if default_category == c { true } else { false }
                            }
                            onchange={
                                let value = c.clone();
                                link.callback(
                                move |_e: Event| {
                                    //let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
                                    Msg::EditCategory(value.clone().to_string())
                                }
                            )}

                        />{c}</label>
                    }
                    </div></div>
                </div>

                <div class="field is-horizontal">
                    <div class="field-label is-normal"><label class="label">{ "Definition:" }</label></div>
                    <div class="field-body"><div class="field"><p class="control">
                    <input
                        placeholder="Definition"
                        class="input is-normal"
                        oninput={ edit_definition }
                    />
                    </p></div></div>
                </div>


              <div class="grid">
              for (idx, l) in langs.iter().enumerate() {
                        <div class="cell">
                        <img src={format!("./resources/svg/{}.svg", l.flag.to_lowercase())} alt={l.clone().name} width="20" />
                        <SubFormDisplay
                            word={ self.words
                                .iter().find(|&x| x.language == *l)
                                .unwrap()
                                .clone() }
                            has_gender={l.has_gender}
                            on_change={
                                let words = self.words.clone();
                                link.callback(move |updated: Word| {
                                    let mut v = words.clone();
                                    v[idx] = updated;
                                    Msg::EditSubForm(v)
                        })}

                        />
                        </div>
                }
                </div></div>

                <div class="field is-grouped">
                  <div class="control">
                    <button class="button is-link"
                       onclick={
                            ctx.link().callback(move |_| Msg::SaveWord())
                       }
                    >{"Submit"}</button>
                  </div>
                  <div class="control">
                    <button class="button"
                        onclick={ctx.link().callback(move |_| Msg::TranslateWord())}
                    >{"⚡"}</button>
                  </div>
                  <div class="control">
                    <button class="button is-link is-light">{"Cancel"}</button>
                  </div>
                </div>
                </>
        }
    }
}

impl Form {
    fn save_words(ws: Vec<Word>) {
       //info!("HERE {:?}", ws);

        spawn_local(async move {
            let json_body = serde_json::to_string(&ws).unwrap();

            let _ = Request::post("/api/v1/save")
                .header("Content-Type", "application/json")
                .body(json_body)
                .expect("Failed to build request")
                .send()
                .await
                .unwrap();
        });
    }
    fn translate_word(w: Vec<Word>, ctx: &Context<Self>) {
        //info!("Translate this: {:?}", w);
        let link = ctx.link().clone();
        spawn_local(async move {
            let json_body = serde_json::to_string(&w).unwrap();

            let tw: Vec<Word> = Request::post("/api/v1/translate")
                .header("Content-Type", "application/json")
                .body(json_body)
                .expect("Failed to build request")
                .send()
                .await
                .unwrap()
                .json()
                .await
                .expect("Failed to parse JSON");
            link.send_message(Msg::SetWords(tw));

        });
    }
}
