use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{HtmlInputElement, InputEvent};
use gloo_net::http::Request;
//use itertools::Itertools;

use common::{word::Word};
//,word::TranslateRequest,word, language, language::Language};

#[derive(Properties, PartialEq)]
pub struct Props {

}
pub struct RandomSerie {
    //words: Vec<Word>,
    picked_serie: Vec<Word>,
    //picked_lang: Language,
    display: String,
    check: String,
    attempt: String,
    result: bool,
    reveal: bool,
}
pub enum Msg {
    //SetWords(Vec<Word>),
    PickSerie(Vec<Word>),
    CheckTranslation(),
    EditTranslation(String),
    MoveNext(),
    ShowTranslation(),
}
impl Component for RandomSerie {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self::fetch_words(ctx);
        Self {
            //words: vec![],
            picked_serie: vec![],
            //picked_lang: Language::default(),
            display: "".to_string(),
            check: "".to_string(),
            attempt: "".to_string(),
            result: false,
            reveal: false,
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MoveNext() => {
                Self::fetch_words(ctx);
                true
            }
            Msg::PickSerie(words) => {
                self.picked_serie = words.clone();
                self.display = words.iter().find(|x| x.language.iso == "EL").unwrap().name.clone();
                self.check = words.iter().find(|x| x.language.iso == "EN").unwrap().name.clone();
                self.reveal = false;
                self.result = false;
                self.attempt = "".to_string();
                true
            }
            Msg::CheckTranslation() => {
                if self.check == self.attempt {
                    self.result = true;
                }
                true
            }
            Msg::EditTranslation(t) => {
                self.attempt = t;
                false
            }
            Msg::ShowTranslation() => {
                if self.reveal { self.reveal = false; }
                else { self.reveal = true; }
                true
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let edit_translation = link.callback(|e: InputEvent| {
            let event: Event = e.dyn_into().unwrap_throw();
            let event_target = event.target().unwrap_throw();
            let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
            Msg::EditTranslation(target.value())
        });
        let mut display_style = "display:none";
        if self.reveal { display_style = ""; }
        //let word_to_search = self.picked_serie.iter().find(|&x| x.language.iso == "EL").unwrap();
        html! {
            <div class="container"><div class="column is-4">
            <div class="card">
             <header class="card-header">

               if self.result { <p class="card-header-title">{ " Congratulation " }</p> }
               else { <p class="card-header-title">{ " Do try... " }</p> }

               <button class="card-header-icon" aria-label="more options">
                 <span class="icon">
                   <i class="fas fa-angle-down" aria-hidden="true"></i>
                 </span>
               </button>
             </header>
             <div class="card-content">
               <p class="is-size-1 has-text-success">
                 { self.display.clone() }
               </p>
               <div class="content" >
                <span
                style={display_style}
                >
                    for w in self.picked_serie.clone() {
                            <p>{ w.language.name }{ " - " }{ w.name }</p>
                    }
                    </span>
               </div>
             </div>
             <footer class="card-footer">
                    <input
                        placeholder="Translate it..."
                        class="input is-normal"
                        oninput={ edit_translation }
                    />
               <a href="#" class="card-footer-item"
                  onclick={
                       ctx.link().callback(move |_| Msg::CheckTranslation())
                  }
               >{"✔"}</a>
               //
               <a href="#" class="card-footer-item"
                  onclick={
                       ctx.link().callback(move |_| Msg::ShowTranslation())
                  }
               >{"🤔"}</a>
               <a href="#" class="card-footer-item"
                  onclick={
                       ctx.link().callback(move |_| Msg::MoveNext())
                  }
               >{"➤"}</a>
             </footer>
            </div>
            </div></div>
        }
    }
}
impl RandomSerie {
    fn fetch_words(ctx: &Context<Self>) {
        let link = ctx.link().clone();
        spawn_local(async move {

            let fetched_words: Vec<Word> = Request::get("http://localhost:9000/random")
                .header("Content-Type", "application/json")
                .send()
                .await
                .unwrap()
                .json()
                .await
                .expect("Failed to parse JSON");

            // Send message back to update component state
            link.send_message(Msg::PickSerie(fetched_words));
        });
    }
}
