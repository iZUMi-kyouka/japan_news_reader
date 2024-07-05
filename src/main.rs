use std::collections::HashMap;
use gloo::console::debug;
use lazy_static::lazy_static;
use yew_router::prelude::*;

pub const DEPLOYMENT_ENV: &str = "local";
pub const NHK_BASE_URL: &str = "https://www3.nhk.or.jp";
lazy_static!(
    static ref BASE_URL: &'static str = {
        if DEPLOYMENT_ENV == "local" {
            "http://192.168.100.15:8000"
        } else if DEPLOYMENT_ENV == "server" {
            "https://aninfo-server.shuttleapp.rs"
        } else {
            "INVALID_DEPLOYMENT_ENVIRONMENT"
        }
    };
);

mod model;
mod context;
mod news_scraper;
mod components;
mod routers;
mod utils;

pub use model::*;
pub use context::*;
pub use news_scraper::{jp_news::*, top_news::*};
pub use components::main_content::MainContent;
pub use components::navbar::NavigationBar;
pub use components::left_menu::*;
use yew_hooks::prelude::use_effect_once;

pub type NewsMetaMap = HashMap<String, String>;

use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let cx = use_reducer(|| AppCtx {
        news_view: NewsView::NHK,
        active_news: NewsCategory::Top{page: 1},
        news_metadata: None
    });

    use_effect_once(
        || {
            let initial_loading = gloo::utils::document().get_element_by_id("initial-loading");
            if let Some(e) = initial_loading {
                e.remove();
                debug!("Removed initial loading.");
            }
            || {}
        }
    );

    html! {
    <>
    <ContextProvider<AppContext> context={cx}>
        <BrowserRouter>
            <LeftMenu/>
            <NavigationBar>
                <div class="main-content-wrapper">
                    <MainContent/>
                </div>
            </NavigationBar>
        </BrowserRouter>
    </ContextProvider<AppContext>>
    </>}
}

fn main() {
    yew::Renderer::<App>::new().render();
}
