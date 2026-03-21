use std::collections::HashMap;

use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

use crate::components::nav::Nav;
use crate::pages::page_not_found::PageNotFound;
use crate::pages::home::Home;
use crate::pages::dictionary::Dictionary;
use crate::pages::random_serie::RandomSerie;

use crate::pages::form::Form;

use crate::pages::haiku_display::HaikuDisplay;
use crate::pages::haiku_form::HaikuForm;
use crate::pages::haikus::Haikus;

use common::word;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/form")]
    Form,
    #[at("/dictionary")]
    Dictionary,
    #[at("/random")]
    RandomSerie,
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
        Route::Home => {
            html! { <Home /> }
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
        Route::RandomSerie => {
            html! { <RandomSerie /> }
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
