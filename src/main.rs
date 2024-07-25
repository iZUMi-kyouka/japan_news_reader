use std::collections::HashMap;
use components::article_list::{NHKCache, YomuriCache};
use gloo::console::debug;
use lazy_static::lazy_static;
use yew_router::prelude::*;

pub const DEPLOYMENT_ENV: &str = "server";

pub const NHK_BASE_URL: &str = "https://www3.nhk.or.jp";

lazy_static!(
    static ref NHKAPI_BASE_URL: &'static str = {
        if DEPLOYMENT_ENV == "local" {
            "http://192.168.100.15:8000/api/v1/nhkreader"
        } else if DEPLOYMENT_ENV == "server" {
            "https://aninfo-server.shuttleapp.rs/api/v1/nhkreader"
        } else {
            "INVALID_DEPLOYMENT_ENVIRONMENT"
        }
    };

    static ref JNAPI_BASE_URL: &'static str = {
        if DEPLOYMENT_ENV == "local" {
            "http://192.168.100.15:8000/api/v1/yomiureader"
        } else if DEPLOYMENT_ENV == "server" {
            "https://aninfo-server.shuttleapp.rs/api/v1/yomiureader"
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
use yewdux::use_store;

#[function_component]
fn App() -> Html {
    let cx = use_reducer(|| AppCtx {
        news_provider: NewsProvider::NHK,
        active_news: NewsCategory::Top{page: 1},
    });

    let (_, dispatch_jn) = use_store::<YomuriCache>();
    let (_, dispatch_nhk) = use_store::<NHKCache>();

    use_effect_once(
        move || {
            let initial_loading = gloo::utils::document().get_element_by_id("initial-loading");
            if let Some(e) = initial_loading {
                e.remove();
                debug!("Removed initial loading.");
            }
            dispatch_jn.set(YomuriCache {
                news_lists: Some(HashMap::new())
            });

            dispatch_nhk.set(NHKCache {
                news_lists: Some(HashMap::new())
            });

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
