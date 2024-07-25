use gloo::{console::info, utils::{document, window}};
use web_sys::{wasm_bindgen::{closure::Closure, JsCast}, Element, HtmlElement};
use yew::prelude::*;
use yew_router::{hooks::use_navigator, navigator::Navigator};
use web_sys::Window;
use yewdux::{use_store, Store};

use crate::{routers::AppRoute, utils::button_control::turn_off_all_navbar_bottom_buttons, AppContext, NHKArticleType, NewsCategory, NewsProvider, YomiuriGenreTag};

#[derive(Properties, PartialEq)]
pub struct NavbarProps {
    #[prop_or_default]
    pub children: Html,
}

#[derive(PartialEq, Debug, Clone)]
pub enum NavbarViewState {
    NHK (NHKArticleType),
    Yomiuri (YomiuriGenreTag)
}

impl Default for NavbarViewState {
    fn default() -> Self {
        NavbarViewState::NHK(NHKArticleType::News)
    }
}


#[derive(Store, PartialEq, Debug, Clone, Default)]
pub struct NavbarState {
    pub navbar_state: NavbarViewState
}

impl NavbarState {
    pub fn update_nb_view(&self, v: NavbarViewState) {

    }
}

#[function_component]
pub fn NavigationBar(props: &NavbarProps) -> Html {
    let nav = use_navigator().unwrap();
    let cx = use_context::<AppContext>().unwrap();
    let nhknews_view = use_state(|| NHKArticleType::News);
    let jnnews_view = use_state(|| YomiuriGenreTag::LatestNews);
    let (state, dispatch) = use_store::<NavbarState>();

    // The actual update process of the button (interacting with the DOM)
    // based on the change on the global NavbarViewState
    use_effect_with(state.clone(), |state| {
        match state.as_ref().navbar_state {
            NavbarViewState::NHK(ref type_) => {
                match type_ {
                    NHKArticleType::News => {
                        if let Some(nb_btn) = document().get_element_by_id("home-btn") {
                            turn_off_all_navbar_bottom_buttons();
                            let _ = nb_btn.set_attribute("active", "true");
                        }
                    },
                    NHKArticleType::Backstory => {
                        if let Some(nb_btn) = document().get_element_by_id("jp-btn") {
                            turn_off_all_navbar_bottom_buttons();
                            let _ = nb_btn.set_attribute("active", "true");
                        }
                    },
                }        
            },
            NavbarViewState::Yomiuri(ref genre) => {
                if let Some(nb_btn) = document().get_element_by_id(&format!("{}-btn", genre)) {
                    turn_off_all_navbar_bottom_buttons();
                    let _ = nb_btn.set_attribute("active", "true");
                }
            },
        }

        });
    // Apply animation on mount
    // {
    //     let cx = cx.clone();
    //     use_effect_with(cx.active_news.clone(), move |_| match &cx.active_news {
    //         NewsCategory::Japan { page: _ } => {
    //             let jp_btn = web_sys::window()
    //                 .unwrap()
    //                 .document()
    //                 .unwrap()
    //                 .get_element_by_id("jp-btn")
    //                 .expect("Japan news button missing");
    //             jp_btn.set_attribute("active", "").unwrap();
    //             let home_btn = web_sys::window()
    //                 .unwrap()
    //                 .document()
    //                 .unwrap()
    //                 .get_element_by_id("home-btn")
    //                 .expect("Japan news button missing");
    //             home_btn.remove_attribute("active").unwrap();
    //         }
    //         NewsCategory::Top{page: _} => {
    //             let jp_btn = web_sys::window()
    //                 .unwrap()
    //                 .document()
    //                 .unwrap()
    //                 .get_element_by_id("jp-btn")
    //                 .expect("Japan news button missing");
    //             jp_btn.remove_attribute("active").unwrap();
    //             let home_btn = web_sys::window()
    //                 .unwrap()
    //                 .document()
    //                 .unwrap()
    //                 .get_element_by_id("home-btn")
    //                 .expect("Japan news button missing");
    //             home_btn.set_attribute("active", "").unwrap();
    //         }
    //     });
    // }

    fn nav_to_yomiuri_genre(g: YomiuriGenreTag, nav: Navigator) -> Callback<MouseEvent> {
        let nav = nav.clone();
        Callback::from(move |_: MouseEvent| {
            nav.push(&AppRoute::YomiuriArticleListGenre { genre: g.clone() });
        })
    }

    let nav_to_japan = {
        let nav = nav.clone();
        Callback::from(move |_: MouseEvent| {
            nav.push(&AppRoute::Japan { page: 1 });
        })
    };

    let nav_to_home = {
        let nav = nav.clone();
        Callback::from(move |_: MouseEvent| {
            nav.push(&AppRoute::DefaultView);
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

    html! {
        <>
            <nav id="navbar-top" class="overlay-shadow">
                <div id="navbar-top__left">
                    <button id="left-menu-button" class="nb-btn-svg"><img class="ui-icon" src="./icons/hb-menu.svg" height="32px"/></button>
                </div>
                <div id="navbar-top__mid">
                    {
                        match &state.as_ref().navbar_state {
                            &NavbarViewState::NHK(_) => html!{<button id="nhk-logo" class="nb-btn"><img class="ui-icon" src="./icons/nhk.svg"/></button>},
                            &NavbarViewState::Yomiuri(_) => html!{<button id="yomiuri-logo" class="nb-btn"><img class="ui-icon" src="./icons/yomiuri.svg" height="32px"/></button>},
                        }
                    }

                </div>
                <div id="navbar-top__right">
                    <button id="theme-button" class="nb-btn-svg"><img class="ui-icon" src="./icons/auto_theme.svg" height="32px"/></button>
                </div>
            </nav>
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
            {props.children.clone()}

            {match state.as_ref().navbar_state {
                NavbarViewState::NHK(_) => {
                    html!{
                        <nav id="navbar-bottom" class="overlay-shadow">
                        <button id="home-btn" class="nb-btn nb-bottom-btn" onclick={nav_to_home}>
                            <img class="nb-btn__icon ui-icon" src="./icons/home.svg"/>
                            <span class="nb-btn__text">{"Home"}</span>
                            <div class="active-indicator"></div>
                        </button>
                        <button id="jp-btn" class="nb-btn nb-bottom-btn" onclick={nav_to_japan}>
                            <img class="nb-btn__icon ui-icon" src="./icons/japan.svg"/>
                            <span class="nb-btn__text">{"Japan"}</span>
                            <div class="active-indicator"></div>
                        </button>
                        </nav>
                    }
                },
                NavbarViewState::Yomiuri(_) => {
                    html!{
                        <nav id="navbar-bottom" class="overlay-shadow">
                        <button id="latestnews-btn" class="nb-btn nb-bottom-btn" onclick={nav_to_yomiuri_genre(YomiuriGenreTag::LatestNews,  nav.clone())}>
                            <img class="nb-btn__icon ui-icon" src="./icons/home.svg"/>
                            <span class="nb-btn__text">{"Home"}</span>
                            <div class="active-indicator"></div>
                        </button>
                        <button id="politics-btn" class="nb-btn nb-bottom-btn"  onclick={nav_to_yomiuri_genre(YomiuriGenreTag::Politics,  nav.clone())}>
                            <img class="nb-btn__icon ui-icon" src="./icons/politics.png"/>
                            <span class="nb-btn__text">{"Politics"}</span>
                            <div class="active-indicator"></div>
                        </button>
                        <button id="society-btn" class="nb-btn nb-bottom-btn"  onclick={nav_to_yomiuri_genre(YomiuriGenreTag::Society,  nav.clone())}>
                            <img class="nb-btn__icon ui-icon" src="./icons/society.png"/>
                            <span class="nb-btn__text">{"Society"}</span>
                            <div class="active-indicator"></div>
                        </button>
                        <button id="business-btn" class="nb-btn nb-bottom-btn"  onclick={nav_to_yomiuri_genre(YomiuriGenreTag::Business,  nav.clone())}>
                            <img class="nb-btn__icon ui-icon" src="./icons/business.png"/>
                            <span class="nb-btn__text">{"Business"}</span>
                            <div class="active-indicator"></div>
                        </button>
                        <button id="world-btn" class="nb-btn nb-bottom-btn"  onclick={nav_to_yomiuri_genre(YomiuriGenreTag::World,  nav.clone())}>
                            <img class="nb-btn__icon ui-icon" src="./icons/world.svg"/>
                            <span class="nb-btn__text">{"World"}</span>
                            <div class="active-indicator"></div>
                        </button>
                        <button id="news-services-btn" class="nb-btn nb-bottom-btn"  onclick={nav_to_yomiuri_genre(YomiuriGenreTag::NewsServices,  nav.clone())}>
                            <img class="nb-btn__icon ui-icon" src="./icons/newspaper.svg"/>
                            <span class="nb-btn__text">{"External"}</span>
                            <div class="active-indicator"></div>
                        </button>
                        <button id="editorial-btn" class="nb-btn nb-bottom-btn"  onclick={nav_to_yomiuri_genre(YomiuriGenreTag::EditorialColumns,  nav.clone())}>
                            <img class="nb-btn__icon ui-icon" src="./icons/editorial.svg"/>
                            <span class="nb-btn__text">{"Editorial"}</span>
                            <div class="active-indicator"></div>
                        </button>
                        <button id="sports-btn" class="nb-btn nb-bottom-btn"  onclick={nav_to_yomiuri_genre(YomiuriGenreTag::Sports,  nav.clone())}>
                            <img class="nb-btn__icon ui-icon" src="./icons/sports.svg"/>
                            <span class="nb-btn__text">{"Sports"}</span>
                            <div class="active-indicator"></div>
                        </button>
                        <button id="science-nature-btn" class="nb-btn nb-bottom-btn"  onclick={nav_to_yomiuri_genre(YomiuriGenreTag::ScienceNature,  nav.clone())}>
                            <img class="nb-btn__icon ui-icon" src="./icons/science.svg"/>
                            <span class="nb-btn__text">{"Science"}</span>
                            <div class="active-indicator"></div>
                        </button>
                        <button id="culture-btn" class="nb-btn nb-bottom-btn"  onclick={nav_to_yomiuri_genre(YomiuriGenreTag::Culture,  nav.clone())}>
                            <img class="nb-btn__icon ui-icon" src="./icons/culture.svg"/>
                            <span class="nb-btn__text">{"Culture"}</span>
                            <div class="active-indicator"></div>
                        </button>
                        <button id="original-btn" class="nb-btn nb-bottom-btn" onclick={nav_to_yomiuri_genre(YomiuriGenreTag::JNSpecialties,  nav.clone())}>
                            <img class="nb-btn__icon ui-icon" src="./icons/world.svg"/>
                            <span class="nb-btn__text">{"Special"}</span>
                            <div class="active-indicator"></div>
                        </button>
                        <button id="features-btn" class="nb-btn nb-bottom-btn"  onclick={nav_to_yomiuri_genre(YomiuriGenreTag::Features,  nav.clone())}>
                            <img class="nb-btn__icon ui-icon" src="./icons/features.svg"/>
                            <span class="nb-btn__text">{"Features"}</span>
                            <div class="active-indicator"></div>
                        </button>
                        </nav>
                    }
                }
            }}

            <script src="./scripts/theme-dropdown.js">
            </script>
        </>
    }
}
