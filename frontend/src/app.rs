use std::collections::HashMap;

use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

use crate::components::nav::Nav;
use crate::pages::page_not_found::PageNotFound;
use crate::pages::home::RandomSerie;
use crate::pages::dictionary::Dictionary;
//use crate::pages::random_serie::RandomSerie;

use crate::pages::form::Form;

use crate::pages::haiku_display::HaikuDisplay;
use crate::pages::haiku_form::HaikuForm;
use crate::pages::haikus::Haikus;

use common::word;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    RandomSerie,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/form")]
    Form,
    #[at("/dictionary")]
    Dictionary,
    #[at("/haiku")]
    Haiku,
    #[at("/haikus")]
    Haikus,
    #[at("/haikuform")]
    HaikuForm,
}
#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Nav />

            <main>
            <div id="content">
                <Switch<Route> render={switch} />
            </div>
            </main>

            <footer class="footer">
                <div class="content has-text-right">

                    <span class="footer_icon"><a href="https://yew.rs">
                    <img
                        src="./resources/yewstack.png"
                        alt="Powered by Yew"
                        width="24"
                        height="24"
                        /></a></span>

                    <span class="footer_icon"><a href="https://rust-lang.org/">
                    <img
                        src="./resources/rust.webp"
                        alt="Powered by Yew"
                        width="24"
                        height="24"
                        /></a></span>

                    <span class="footer_icon"><a href="https://github.com/lunarust/flashcards">
                      <img
                        src="./resources/GitHub_Invertocat_White.svg"
                        alt="GitHub"
                        width="24"
                        height="24" /></a></span>

                    <span class="footer_icon"><a href="https://bulma.io">
                      <img
                        src="./resources/BulmaIcon.png"
                        alt="Made with Bulma"
                        width="15"
                        height="24" />
                    </a></span>

                    <span class="footer_icon">
                      <img
                        src="./resources/logo.svg"
                        alt="Kappa"
                        width="30"
                        height="28" /></span>
                </div>
            </footer>

            <div id="logo">
            <span class="footer_logo">
              <img
                src="./resources/kaoru.png"
                alt="Kaoru"
                width="75"
                height="68" /></span>
            </div>

        </BrowserRouter>
    }
}

#[derive(Properties, PartialEq, Eq, Debug)]
pub struct ServerAppProps {
    pub url: AttrValue,
    pub queries: HashMap<String, String>,
}


#[function_component]
pub fn ServerApp(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history
        .push_with_query(&*props.url, &props.queries)
        .unwrap();

    html! {
        <Router history={history}>
            //<Nav />

            <main>
                <Switch<Route> render={switch} />
            </main>

            <footer class="footer">
                <div class="content has-text-centered">
                    <a href="https://bulma.io">
                      <img
                        src="./BulmaIcon.png"
                        alt="Made with Bulma"
                        width="31"
                        height="48" />
                    </a>
                    <a href="https://yew.rs">
                    <img
                        src="https://avatars.githubusercontent.com/u/49116234?s=48&v=4"
                        alt="Powered by Yew"
                        width="48"
                        height="48"
                        /></a>
                    <a href="https://github.com/lunarust">
                      <img
                        src="../GitHub_Invertocat_Black_Clearspace.png"
                        alt="GitHub"
                        width="48"
                        height="48" /></a>
                </div>
            </footer>

        </Router>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::RandomSerie => {
            html! { <RandomSerie /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
        Route::Form => {
            html! { <Form set={word::build_default_serie()} /> }
        }
        Route::Dictionary => {
            html! { <Dictionary /> }
        }
        Route::Haiku => {
            html! { <HaikuDisplay /> }
        }
        Route::HaikuForm => {
            html! { <HaikuForm /> }
        }
        Route::Haikus => {
            html! { <Haikus /> }
        }
    }
}
