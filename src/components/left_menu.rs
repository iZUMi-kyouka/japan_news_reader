use gloo::utils::document;
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use yewdux::use_store;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/scripts/left-menu.js")]
extern "C" {
    fn closeLeftMenu();
}

use crate::{components::navbar::NavbarState, routers::AppRoute, utils::button_control::turn_off_all_navbar_bottom_buttons, AppContext, AppCtx, NewsProvider};

use super::navbar::NavbarViewState;

#[function_component]
pub fn LeftMenu() -> Html {
    let nav = use_navigator().unwrap();
    let (state, dispatch) = use_store::<NavbarState>();
    
    let change_news_provider_to_nhk = {
        let state = state.clone();
        let nav = nav.clone();
        Callback::from(move |_: MouseEvent| {
            if let &NavbarViewState::Yomiuri(_) = &state.as_ref().navbar_state {
                turn_off_all_navbar_bottom_buttons();
                nav.push(&AppRoute::DefaultView);
                gloo::console::info!("changing news to NHK");
                let jn_btn = document().get_element_by_id("yomiuri-logo-left-menu").expect("Failed to get JN left menu btn");
                let nhk_btn = document().get_element_by_id("nhk-logo-left-menu").expect("Failed to get NHK left menu btn");
                let _ = nhk_btn.set_attribute("disabled", "true");
                let _ = jn_btn.remove_attribute("disabled");
                // closeLeftMenu();
            }
        })
    };

    let change_news_provider_to_jn = {
        let state = state.clone();
        let nav = nav.clone();
        Callback::from(move |_: MouseEvent| {
            if let &NavbarViewState::NHK(_) = &state.as_ref().navbar_state{
                nav.push(&AppRoute::YomiuriLatestNews);
                gloo::console::info!("changing news to JapanNews");
                let jn_btn = document().get_element_by_id("yomiuri-logo-left-menu").expect("Failed to get JN left menu btn");
                let nhk_btn = document().get_element_by_id("nhk-logo-left-menu").expect("Failed to get NHK left menu btn");
                let _ = jn_btn.set_attribute("disabled", "true");
                let _ = nhk_btn.remove_attribute("disabled");
                // closeLeftMenu();
            }
        })
    };

    let block_onscroll = Callback::from(|e: WheelEvent| {
        e.prevent_default();
        e.stop_propagation();
    });

    html!{
        <>
        <div id="left-menu" onwheel={block_onscroll.clone()}>
            <button id="left-menu-close-btn" class="nb-btn"><img class="ui-icon" src="./icons/close.svg" width="40px" height="40px"/></button>
            <div id="left-menu__news-provider">
                <span class="left-menu__item__title">{"Select news provider"}</span>
                <button id="nhk-logo-left-menu" class="nb-btn" onclick={change_news_provider_to_nhk}><img class="ui-icon" src="./icons/nhk.svg"/></button>
                <button id="yomiuri-logo-left-menu" class="nb-btn" onclick={change_news_provider_to_jn}><img class="ui-icon" src="./icons/yomiuri.svg"/></button>
            </div>
        </div>
        <div id ="left-menu-bg" onwheel={block_onscroll}></div>
        <script src="./scripts/left-menu.js"></script>
        </>
    }
}