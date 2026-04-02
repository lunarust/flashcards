use yew::prelude::*;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen_futures::spawn_local;
use gloo_net::http::Request;
//use log::info;
use web_sys::HtmlInputElement;

use common::{haiku::*};
use crate::pages::haiku_line_form::SubFormDisplay;

const BACKEND: &str = "http://localhost:9000";

#[derive(Properties, PartialEq)]
pub struct Props {

}

#[derive(Properties, PartialEq)]
pub struct HaikuForm {
    pub new_haiku: Haiku,
}
pub enum Msg {
    EditTitle(String),
    EditCreated(String),
    EditAuthor(String),
    EditDeck(String),
    SaveHaiku(),
    EditSubForm(Vec<HaikuLine>),
    AddNewLine(),
}
impl Component for HaikuForm {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { new_haiku: Haiku::default()
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SaveHaiku() => {
                let my_haiku = self.new_haiku.clone();
                Self::save_haiku(my_haiku);
                false
            }
            Msg::AddNewLine() => {
                self.new_haiku.haiku_line.push(HaikuLine::default());
                true
            }
            Msg::EditTitle(a) => {
                self.new_haiku.title = a;
                false
            }
            Msg::EditAuthor(a) => {
                self.new_haiku.author = a;
                false
            }
            Msg::EditCreated(a) => {
                self.new_haiku.created = a;
                false
            }
            Msg::EditDeck(a) => {
                self.new_haiku.deck = a;
                false
            }
            Msg::EditSubForm(vhl) => {
                self.new_haiku.haiku_line = vhl;
                true
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let new_haiku = self.new_haiku.clone();
        let new_haiku_line = new_haiku.haiku_line.clone();


       let edit_created = link.callback(|e: InputEvent| {
           let event: Event = e.dyn_into().unwrap_throw();
           let event_target = event.target().unwrap_throw();
           let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
           Msg::EditCreated(target.value())
       });

       let edit_title = link.callback(|e: InputEvent| {
           let event: Event = e.dyn_into().unwrap_throw();
           let event_target = event.target().unwrap_throw();
           let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
           Msg::EditTitle(target.value())
       });

       let edit_author = link.callback(|e: InputEvent| {
           let event: Event = e.dyn_into().unwrap_throw();
           let event_target = event.target().unwrap_throw();
           let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
           Msg::EditAuthor(target.value())
       });

       let edit_deck = link.callback(|e: InputEvent| {
           let event: Event = e.dyn_into().unwrap_throw();
           let event_target = event.target().unwrap_throw();
           let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
           Msg::EditDeck(target.value())
       });

        html!{
            <>
            <div class="fixed-grid">
                <div class="field-label is-normal"><label class="label">{ "Author:" }</label></div>
                <div class="field-body"><div class="field"><p class="control">
                <input
                    placeholder="Author..."
                    class="input is-normal"
                    oninput={ edit_author }
                    />
                </p></div></div>
                <div class="field-label is-normal"><label class="label">{ "Title:" }</label></div>
                <div class="field-body"><div class="field"><p class="control">
                <input
                    placeholder="Title..."
                    class="input is-normal"
                    oninput={ edit_title }
                    />
                </p></div></div>
                <div class="field-label is-normal"><label class="label">{ "Created:" }</label></div>
                <div class="field-body"><div class="field"><p class="control">
                <input
                    placeholder="Created, approximatively..."
                    class="input is-normal"
                    oninput={ edit_created }
                    />
                </p></div></div>

                <div class="field-label is-normal"><label class="label">{ "Deck:" }</label></div>
                <div class="field-body"><div class="field"><p class="control">
                <input
                    placeholder="Deck"
                    class="input is-normal"
                    oninput={ edit_deck }
                    />
                </p></div></div>

                // Lines
                <div class="grid">
                for (idx, hl) in new_haiku_line.iter().enumerate() {
                        <div class="cell">
                        <SubFormDisplay
                            haikuline={ new_haiku_line[idx].clone() }
                            order={ idx as i32 }
                            on_change={
                                let nhl = new_haiku_line.clone();
                                link.callback(move |updated: HaikuLine| {
                                    let mut nhl = nhl.clone();
                                    nhl[idx] = updated;
                                    nhl[idx].order = idx as i32;
                                    Msg::EditSubForm(nhl)
                            })}
                        />
                        </div>
                }

                </div>

                // Submit
                <div class="field is-grouped">
                  <div class="control">
                    <button class="button is-info"
                        onclick={
                                ctx.link().callback(move |_| Msg::AddNewLine())
                        }>{ "+" }</button>

                    <button class="button is-info"
                       onclick={
                            ctx.link().callback(move |_| Msg::SaveHaiku())
                       }
                    >{"Submit"}</button>
                  </div></div>
            </div>
            </>
        }
    }
}

impl HaikuForm {
    fn save_haiku(h: Haiku) {
        spawn_local(async move {
            let json_body = serde_json::to_string(&h).unwrap();
            let _ = Request::post(format!("{}/haiku", BACKEND).as_str())
                .header("Content-Type", "application/json")
                .body(json_body)
                .expect("Failed to build request")
                .send()
                .await
                .unwrap();
        });
    }
}
