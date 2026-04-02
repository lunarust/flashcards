use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{HtmlInputElement, InputEvent};
use gloo_net::http::Request;
//use itertools::Itertools;

use common::haiku::*;

const BACKEND: &str = "http://localhost:9000";

#[derive(Properties, PartialEq)]
pub struct Props {

}
pub struct Haikus {
    haikus: Vec<Haiku>,
    filtered_haikus: Vec<Haiku>,
    filter: String,
    selected_haiku: Haiku,
}
pub enum Msg {
    SetHaikus(Vec<Haiku>),
    SelectHaiku(Haiku),
    FilterHaikus(String),
}
impl Component for Haikus {
    type Message = Msg;
    type Properties = Props;
    fn create(ctx: &Context<Self>) -> Self {
        Self::fetch_haikus(ctx);
        Self {
            haikus: vec![],
            filtered_haikus: vec![],
            filter: "".to_string(),
            selected_haiku: Haiku::default(),
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetHaikus(mut h) => {
                self.filtered_haikus = h.clone();
                self.haikus = h;
                true
            },
            Msg::SelectHaiku(h) => {
                self.selected_haiku = h;
                true
            },
            Msg::FilterHaikus(f) => {
                self.filtered_haikus = self.haikus.clone()
                    .into_iter()
                    .filter(|h| h.deck.to_lowercase().contains(&f.to_lowercase()))
                    .collect();
                true
            },
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let list_haikus = self.filtered_haikus.clone();
        let is_selected = self.selected_haiku.clone();

        let oninput = link.callback(|e: InputEvent| {
            let event: Event = e.dyn_into().unwrap_throw();
            let event_target = event.target().unwrap_throw();
            let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
            Msg::FilterHaikus(target.value())
        });

        html! {
            <>
            <table class="table">
            <thead>
                <tr>
                    <th>{ "..." }</th>
                    <th>{ "Author" }</th>
                    <th>{ "Title" }</th>
                    <th>{ "Haiku" }</th>
                    <th>{ "Created" }</th>
                    <th>
                        <input
                            placeholder="Search"
                            class="input is-normal"
                            oninput={ oninput }
                        />
                    </th>
                </tr>
            </thead>
            <tbody>
                for ha in list_haikus.clone().iter() {
                    <tr
                        onclick={
                            let sel_ha = ha.clone();
                            ctx.link().callback(move |_| Msg::SelectHaiku(sel_ha.clone()))
                        }
                        class={ if &is_selected == ha { "tr-selected" } else { "" }}
                    >
                        <td>{ ha.id.clone() }</td>
                        <td>{ ha.author.clone() }</td>
                        <td>{ ha.title.clone() }</td>
                        <td>
                            for hl in ha.haiku_line.iter() {
                                {hl.line.clone()}
                            }
                        </td>
                        <td>{ ha.created.clone() }</td>
                        <td>{ ha.deck.clone() }</td>
                    </tr>
                }
            </tbody>
            </table>
            </>
        }
    }
}
impl Haikus {
    fn fetch_haikus(ctx: &Context<Self>) {
        let link = ctx.link().clone();
        spawn_local(async move {
            let fetched_haikus: Vec<Haiku> = Request::get(format!("{}/haikus", BACKEND).as_str())
                .header("Content-Type", "application/json")
                .send()
                .await
                .unwrap()
                .json()
                .await
                .expect("Failed to parse JSON");

            // Send message back to update component state
            link.send_message(Msg::SetHaikus(fetched_haikus));
        });
    }
}
