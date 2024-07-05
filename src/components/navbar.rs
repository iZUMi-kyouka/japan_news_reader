use std::sync::Arc;

use gloo::utils::{document, window};
use web_sys::{wasm_bindgen::{closure::Closure, JsCast}, Element};
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use web_sys::Window;

use crate::{routers::AppRoute, AppContext, NewsCategory, NewsView};

#[derive(Properties, PartialEq)]
pub struct NavbarProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn NavigationBar(props: &NavbarProps) -> Html {
    let nav = use_navigator().unwrap();
    let cx = use_context::<AppContext>().unwrap();

    // Apply animation on mount
    {
        let cx = cx.clone();
        use_effect_with(cx.active_news.clone(), move |_| match &cx.active_news {
            NewsCategory::Japan { page: _ } => {
                let jp_btn = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_element_by_id("jp-btn")
                    .expect("Japan news button missing");
                jp_btn.set_attribute("active", "").unwrap();
                let home_btn = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_element_by_id("home-btn")
                    .expect("Japan news button missing");
                home_btn.remove_attribute("active").unwrap();
            }
            NewsCategory::Top{page: _} => {
                let jp_btn = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_element_by_id("jp-btn")
                    .expect("Japan news button missing");
                jp_btn.remove_attribute("active").unwrap();
                let home_btn = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_element_by_id("home-btn")
                    .expect("Japan news button missing");
                home_btn.set_attribute("active", "").unwrap();
            }
        });
    }

    let nav_to_japan = {
        let cx = cx.clone();
        let nav = nav.clone();
        Callback::from(move |_: MouseEvent| {
            // if let Some(main) = document().get_element_by_id("main-content") {
            //     if let NewsCategory::Top { page: _ } = cx.active_news {
            //         main.remove_attribute("active");
            //     }
            // } 
            
            nav.push(&AppRoute::Japan { page: 1 });
        })
    };

    let nav_to_home = {
        let nav = nav.clone();
        Callback::from(move |_: MouseEvent| {
            // if let Some(main) = document().get_element_by_id("main-content") {
            //     if let NewsCategory::Japan { page: _ } = cx.active_news {
            //         main.remove_attribute("active");
            //     }
            // } 

            nav.push(&AppRoute::HomeDefault);
        })
    };

    let force_dark_theme = {
        Callback::from(|e: MouseEvent| {
            let Some(doc_element) = document().document_element() else {
                gloo::console::warn!("Document element is missing.");
                return () };
            let _ = doc_element.set_attribute("data-theme", "dark");
        })
    };

    let force_light_theme = {
        Callback::from(|e: MouseEvent| {
            let Some(doc_element) = document().document_element() else {
                gloo::console::warn!("Document element is missing.");
                return () };
            let _ = doc_element.set_attribute("data-theme", "light");
        })
    };

    let set_auto_theme = {
        Callback::from(|e: MouseEvent| {
            let Some(doc_element) = document().document_element() else {
                gloo::console::warn!("Document element is missing.");
                return () };
            let _ = doc_element.remove_attribute("data-theme");
        })
    };

    let open_left_menu = {
        Callback::from(|e: MouseEvent| {
            let Some(left_menu) = document().get_element_by_id("left-menu") else { return () };
            left_menu.set_attribute("active", "true");
        })
    };

    html! {
        <>
            <nav id="navbar-top" class="overlay-shadow">
                <div id="navbar-top__left">
                    <button id="left-menu-button" class="nb-btn-svg" onclick={open_left_menu}><img class="ui-icon" src="./icons/hb-menu.svg" height="32px"/></button>
                </div>
                <div id="navbar-top__mid">
                    <button id="nhk-logo" class="nb-btn"><img src="./icons/nhk.svg"/></button>
                </div>
                <div id="navbar-top__right">
                    <button id="theme-button" class="nb-btn-svg"><img class="ui-icon" src="./icons/auto_theme.svg" height="32px"/></button>
                    <div id="theme-dropdown">
                        <button class="theme-dropdown__item" onclick={set_auto_theme}>
                            <div class="ui-icon-wrapper"><img class="ui-icon" src="./icons/auto_theme.svg" height="32px"/></div>
                            <div class="theme-dropdown-item__label"><span>{"Auto"}</span></div>
                        </button>
                        <button class="theme-dropdown__item" onclick={force_dark_theme}>
                        <div class="ui-icon-wrapper"><img class="ui-icon" src="./icons/dark_theme.svg" height="28px"/></div>
                            <div class="theme-dropdown-item__label"><span>{"Dark"}</span></div>
                        </button>
                        <button class="theme-dropdown__item" onclick={force_light_theme}>
                        <div class="ui-icon-wrapper"><img class="ui-icon" src="./icons/light_theme.svg" height="32px"/></div>
                            <div class="theme-dropdown-item__label"><span>{"Light"}</span></div>
                        </button>
                    </div>
                </div>
            </nav>

            {props.children.clone()}

            {match &(*cx).news_view {
                &NewsView::NHK => {
                    html!{
                        <nav id="navbar-bottom" class="overlay-shadow">
                        <button id="home-btn" class="nb-btn" onclick={nav_to_home}>
                            <img class="nb-btn__icon" src="./icons/home.svg"/>
                            <span class="nb-btn__text">{"Home"}</span>
                        </button>
                        <button id="jp-btn" class="nb-btn" onclick={nav_to_japan}>
                            <img class="nb-btn__icon" src="./icons/japan.svg"/>
                            <span class="nb-btn__text">{"Japan"}</span>
                        </button>
                        </nav>
                    }
                },
                &NewsView::Yomiuiri => {
                    html!{
                        <nav id="navbar-bottom" class="overlay-shadow">
                        <button id="home-btn" class="nb-btn" onclick={nav_to_home}>
                            <img class="nb-btn__icon" src="./icons/home.svg"/>
                            <span class="nb-btn__text">{"Home"}</span>
                        </button>
                        <button id="jp-btn" class="nb-btn" onclick={nav_to_japan}>
                            <img class="nb-btn__icon" src="./icons/japan.svg"/>
                            <span class="nb-btn__text">{"Japan"}</span>
                        </button>
                        <button id="home-btn" class="nb-btn">
                        <img class="nb-btn__icon" src="./icons/home.svg"/>
                        <span class="nb-btn__text">{"Home"}</span>
                    </button>
                    <button id="jp-btn" class="nb-btn">
                        <img class="nb-btn__icon" src="./icons/japan.svg"/>
                        <span class="nb-btn__text">{"Japan"}</span>
                    </button>
                        </nav>
                    }
                }
            }}

            <script>
            {r#"
window.setTimeout(() => {
    const themeBtn = document.getElementById("theme-button");
    const themeDropdown = document.getElementById("theme-dropdown")

    const closeThemeDropdown = function(event) {
        const eventTarget = event.target;
        if (!themeDropdown.contains(eventTarget) && !(themeDropdown === eventTarget))  {
            console.log("removing active attribute");
            themeDropdown.removeAttribute("active");
            window.removeEventListener('click', closeThemeDropdown);
        }
    }

    themeBtn.addEventListener('click', () => {
        console.log("clicked.");
        if (themeDropdown.getAttribute("active")) {
            themeDropdown.removeAttribute("active");
        } else {
            themeDropdown.setAttribute("active", "true");
            window.setTimeout(() => {
                window.addEventListener('click', closeThemeDropdown);
            }, 25);
        }

})}, 25);"#}
            </script>
        </>
    }
}
