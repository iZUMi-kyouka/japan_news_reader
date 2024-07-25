use std::collections::HashMap;
use std::rc::Rc;
use std::str::FromStr;

use _ArticleListProps::list_metadata;
use chrono::{format, NaiveDate};
use gloo::console::{info, log};
use gloo::utils::document;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew::suspense::{use_future, use_future_with};
use yew_router::hooks::use_navigator;
use yew_router::navigator::Navigator;
use yewdux::{dispatch, use_store, Store};
use crate::components::navbar::NavbarViewState;
use crate::routers::AppRoute;
use crate::utils::button_control::turn_off_all_navbar_bottom_buttons;
use crate::{retrieve_japan_news, retrieve_top_news, AppContext, ArticleListMetadata, JNArticleMeta, NHKArticleMeta, NHKArticlePart, NHKArticleType, YomiuriGenreTagSub, JNAPI_BASE_URL};
use crate::{components::loading::Loading, NewsProvider, YomiuriGenre, YomiuriGenreTag};
use crate::model::SubGenre;
use crate::components::subgenre_button::SubgenreButton;
use super::navbar::NavbarState;

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct ArticleListProps {
    pub list_metadata: ArticleListMetadata
}

#[derive(Store, Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct YomuriCache {
    pub news_lists: Option<HashMap<YomiuriGenreTagSub, Vec<JNArticleMeta>>>
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum NHKArticle {
    News{
        title: String,
        date: NaiveDate,
        text: Vec<String>
    },
    Backstory(Vec<NHKArticlePart>)
}

fn yomiuri_genre_list_to_html(article_list: &[JNArticleMeta], nav: Navigator) -> Html {
    article_list.iter().map(|meta| {
        let nav = nav.clone();
        let url = meta.url.clone();
        let view_article = Callback::from(move |_: MouseEvent| {
            let mut link = url.as_str()[32..&url.len()-1].split('/');
            let genre = YomiuriGenreTag::from_str(link.next().unwrap()).unwrap();
            let (subgenre, article_id) = (link.next().unwrap(), link.next().unwrap());
            nav.push(&AppRoute::YomiuriArticle { genre, subgenre: subgenre.to_string(), article_id: article_id.to_string() });
        });

        html!{
            <article class="news-article flex-col" onclick={view_article}>
                <span class="news-article__title link-deco kb-foc" tabindex=0>{meta.title.as_ref().map(|s| s.as_str()).unwrap_or("<TITLE UNAVAILABLE>")}</span>
                <span class="news-article__date">{meta.date.as_ref().map(|d|                         
                    d.format("%d %B %Y").to_string()).unwrap_or("".to_string())}</span>
            </article>
        }
    }).collect::<Html>()
}

fn nhk_genre_list_to_html_backstory(article_list: &[NHKArticleMeta], nav: Navigator) -> Html {
    article_list.iter()
        .map(|news_meta| {
            let view_article = {
                let nav = nav.clone();
                let article_title = news_meta.title.clone().unwrap_or("<TITLE UNAVAILABLE>".to_string());
                let news_id = news_meta.relative_url.as_str().split('/').skip(5).next().map(|s| s.to_string()).unwrap_or_default();
                Callback::from(move |_: MouseEvent| {
                    nav.push(&AppRoute::JapanNewsArticle { article_id: String::from(&news_id), })
                })
            };
            html!{
            <article class="news-article flex-col" onclick={view_article}>
                <span class="news-article__title link-deco kb-foc" tabindex=0 >{news_meta.title.as_ref().map(|s| s.as_str()).unwrap_or("<TITLE UNAVAILABLE>")}</span>
                <span class="news-article__date">{news_meta.date.as_ref().map(|d|                         
                    d.format("%d %B %Y").to_string()).unwrap_or("".to_string())}</span>
            </article>
            }
        })
        .collect::<Html>()
}

fn nhk_genre_list_to_html_news(article_list: &[NHKArticleMeta], nav: Navigator) -> Html {
    article_list.iter()
        .map(|news_meta| {
            let view_article = {
                let nav = nav.clone();
                let article_title = news_meta.title.clone().unwrap_or("<TITLE UNAVAILABLE>".to_string());
                let news_id = news_meta.relative_url.as_str().split('/').skip(4).next().map(|s| s.to_string()).unwrap_or_default();
                Callback::from(move |_: MouseEvent| {
                    nav.push(&AppRoute::TopNewsArticle { article_id: String::from(&news_id), })
                })
            };
            html!{
            <article class="news-article flex-col" onclick={view_article}>
                <span class="news-article__title link-deco kb-foc" tabindex=0 >{news_meta.title.as_ref().map(|s| s.as_str()).unwrap_or("<TITLE UNAVAILABLE>")}</span>
                <span class="news-article__date">{news_meta.date.as_ref().map(|d|                         
                    d.format("%d %B %Y").to_string()).unwrap_or("".to_string())}</span>
            </article>
            }
        })
        .collect::<Html>()
}

#[derive(Store, Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct NHKCache {
    pub news_lists: Option<HashMap<NHKArticleType, Vec<NHKArticleMeta>>>
}

#[function_component]
pub fn ContentSubgenre(props: &ArticleListProps) -> HtmlResult {
    let cx: AppContext = use_context().unwrap();
    let nav = use_navigator().unwrap();

    // Onrender animation
    {
        use_effect(move || {
            // log!("use effect ran");

            wasm_bindgen_futures::spawn_local(async move {
                let element = document().get_element_by_id("main-content");
                while let None = element {
                    gloo::timers::future::sleep(std::time::Duration::from_millis(10)).await;
                    
                }

                gloo::timers::future::sleep(std::time::Duration::from_millis(10)).await;
                let element = element.unwrap();
                let _ = element.set_attribute("active", "true");
            });

            || {
                // log!("cleanup ran.");
                let main_element = document().get_element_by_id("main-content");
                if let Some(main) = main_element {
                    // log!("cleanup succesful: removed `active` from main-content");
                    let _ = main.remove_attribute("active");
                } else {
                    // log!("cleanup failed: main-content is missing.");
                }
            }
        });
    }

    let article_list_metadata = props.list_metadata.clone();
    let result = use_future_with(props.list_metadata.clone(), move |_| async {
        match article_list_metadata {
            ArticleListMetadata::NHK { type_ } => {
                panic!("unimplemented");
            },
            ArticleListMetadata::Yomiuri { genre, subgenre, page } => {
                let url = format!("{}/{}/{}/{}", *JNAPI_BASE_URL, genre, subgenre.unwrap().link, page.unwrap_or(1));
                let request_result = reqwasm::http::Request::get(&url).send().await;
                let Ok(request_result) = request_result else { 
                    let e = request_result.unwrap_err();
                    return Err(e);
                };

                let jn_article_meta = request_result.json::<Vec<JNArticleMeta>>().await;
                let Ok(jn_article_meta) = jn_article_meta else {
                    let e = jn_article_meta.unwrap_err();
                    return Err(e);
                };
                
                Ok(jn_article_meta)
            }
        }
    })?;

    let jn_article_meta = result.as_ref().unwrap();
    let html_result = jn_article_meta.iter().map(|meta| {
        let nav = nav.clone();
        let url = meta.url.clone();
        let view_article = Callback::from(move |_: MouseEvent| {
            let mut link = url.as_str()[32..&url.len()-1].split('/');
            let genre = YomiuriGenreTag::from_str(link.next().unwrap()).unwrap();
            let (subgenre, article_id) = (link.next().unwrap(), link.next().unwrap());
            nav.push(&AppRoute::YomiuriArticle { genre, subgenre: subgenre.to_string(), article_id: article_id.to_string() });
        });

        html!{
            <article class="news-article flex-col" onclick={view_article}>
                <span class="news-article__title link-deco kb-foc" tabindex=0>{meta.title.as_ref().map(|s| s.as_str()).unwrap_or("<TITLE UNAVAILABLE>")}</span>
                <span class="news-article__date">{meta.date.as_ref().map(|d|                         
                    d.format("%d %B %Y").to_string()).unwrap_or("".to_string())}</span>
            </article>
        }
    }).collect::<Html>();

    Ok(html_result)
}

#[function_component]
pub fn ArticleSubGenreList(props: &ArticleListProps) -> Html {
    let (state, dispatch) = use_store::<NavbarState>();
    // Set the correct navbar state
    {
        let dispatch = dispatch.clone();
        use_effect_with(props.clone(), move |props| {
            match props.list_metadata {
                ArticleListMetadata::NHK { ref type_ } => {
                    dispatch.set(
                        NavbarState { navbar_state: NavbarViewState::NHK(type_.clone()) }
                    );
                },
                ArticleListMetadata::Yomiuri { ref genre, ref subgenre, ref page } => {
                    dispatch.set( NavbarState { navbar_state: NavbarViewState::Yomiuri(genre.clone()) }
                    );
                },
            }}
        );
    }

    html!{
        <Suspense fallback={html!{<Loading/>}}>
            <main id="main-content">
                <ContentSubgenre list_metadata={props.list_metadata.clone()}/>
                {
                    if let ArticleListMetadata::Yomiuri{ genre, subgenre, page } = &props.list_metadata {
                        html!{
                            <>
                                <SubgenreButton genre={genre.clone()}/>
                                <div id="breadcrumb">
                                    <div class="breadcrumb__wrapper">
                                    <span class="breadcrumb-item">{format!("{}{}", genre.as_str().chars().next().map(|s| s.to_ascii_uppercase()).unwrap(), &genre.as_str()[1..])}</span>
                                    <span class="breadcrumb-item">{subgenre.as_ref().unwrap().name.as_str()}</span>
                                    </div>
                                </div>
                            </>}
                    } else {
                        html!{}
                    }
                }
            </main>
            <style>
            {"
            #subgenre-button {
                bottom: 6rem;
            }
            
            #subgenre-dropdown {
                bottom: 9.5rem;
            }

            div.main-content-wrapper {
                margin-bottom: 1.5rem;
            }
            "}
            </style>
        </Suspense>
    }
}

#[function_component]
pub fn ContentGenre(props: &ArticleListProps) -> HtmlResult {
    let (state_jn, dispatch_jn) = use_store::<YomuriCache>();
    let (state_nhk, dispatch_nhk) = use_store::<NHKCache>();
    let nav = use_navigator().unwrap();

    // Onrender animation
    {
        use_effect(move || {
            // log!("use effect ran");

            wasm_bindgen_futures::spawn_local(async move {
                let element = document().get_element_by_id("main-content");
                while let None = element {
                    gloo::timers::future::sleep(std::time::Duration::from_millis(10)).await;
                    
                }

                gloo::timers::future::sleep(std::time::Duration::from_millis(10)).await;
                let element = element.unwrap();
                let _ = element.set_attribute("active", "true");
            });

            || {
                // log!("cleanup ran.");
                let main_element = document().get_element_by_id("main-content");
                if let Some(main) = main_element {
                    // log!("cleanup succesful: removed `active` from main-content");
                    let _ = main.remove_attribute("active");
                } else {
                    // log!("cleanup failed: main-content is missing.");
                }
            }
        });
    }
    
    let article_list_metadata = props.list_metadata.clone();
    let result = {
        let state_jn = Rc::clone(&state_jn);
        let state_nhk = Rc::clone(&state_nhk);
        let dispatch_jn = dispatch_jn.clone();
        let dispatch_nhk = dispatch_nhk.clone();
        use_future_with(props.list_metadata.clone(), move |_| async move {
            match article_list_metadata {
                ArticleListMetadata::NHK { type_ } => {
                    let result = match type_ {
                        NHKArticleType::Backstory => {
                            // Check in-memory cache
                            if let Some(ref hash_map) = (*state_nhk).news_lists {
                                if let Some(article_list) = hash_map.get(&NHKArticleType::Backstory) {
                                    // Article list found in in-memory cache
                                    return Ok(nhk_genre_list_to_html_backstory(article_list, nav));
                                } else {
                                    // Article list not found in in-memory cache. Fetch article list and update in-memory cache.
                                    let request_result = retrieve_japan_news(1).await;
                                    if let Ok(article_list) = request_result {
                                        dispatch_nhk.reduce_mut(|nhk_cache| {
                                            if let Some(ref mut hash_map) = nhk_cache.news_lists {
                                                hash_map.insert(NHKArticleType::Backstory, article_list.list.clone());
                                            }
                                        });
                                        return Ok(nhk_genre_list_to_html_backstory(&article_list.list, nav));
                                    } else {
                                        panic!()
                                    }
                                } 
                            }

                            let request_result = retrieve_japan_news(1).await;
                            if let Ok(article_list) = request_result {
                                let mut hash_map = HashMap::new();
                                dispatch_nhk.reduce_mut(|nhk_cache| {
                                    hash_map.insert(NHKArticleType::Backstory, article_list.list.clone());
                                    dispatch_nhk.set(NHKCache { news_lists: Some(hash_map) });
                                });
                                return Ok(nhk_genre_list_to_html_backstory(&article_list.list, nav));
                            } else {
                                panic!()
                            }
                        },
                        NHKArticleType::News => {
                            // Check in-memory cache
                            if let Some(ref hash_map) = (*state_nhk).news_lists {
                                if let Some(article_list) = hash_map.get(&NHKArticleType::News) {
                                    // Article list found in in-memory cache
                                    return Ok(nhk_genre_list_to_html_news(article_list, nav));
                                } else {
                                    // Article list not found in in-memory cache. Fetch article list and update in-memory cache.
                                    let request_result = retrieve_top_news(1).await;
                                    if let Ok(article_list) = request_result {
                                        dispatch_nhk.reduce_mut(|nhk_cache| {
                                            if let Some(ref mut hash_map) = nhk_cache.news_lists {
                                                hash_map.insert(NHKArticleType::News, article_list.clone());
                                            }
                                        });
                                        return Ok(nhk_genre_list_to_html_news(&article_list, nav));
                                    } else {
                                        panic!()
                                    }
                                } 
                            }

                            let request_result = retrieve_top_news(1).await;
                            if let Ok(article_list) = request_result {
                                let mut hash_map = HashMap::new();
                                dispatch_nhk.reduce_mut(|nhk_cache| {
                                    hash_map.insert(NHKArticleType::News, article_list.clone());
                                    dispatch_nhk.set(NHKCache { news_lists: Some(hash_map) });
                                });
                                Ok(nhk_genre_list_to_html_news(&article_list, nav))
                            } else {
                                panic!()
                            }
                        }
                    };

                    result
                },
                ArticleListMetadata::Yomiuri { genre, subgenre, page } => {
                    // Check in-memory cache
                    if let Some(ref hash_map) = (*state_jn).news_lists {
                        if let Some(article_list) = hash_map.get(&YomiuriGenreTagSub { genre: genre.clone(), subgenre: subgenre.clone() }) {
                            // Article list found in in-memory cache
                            return Ok(yomiuri_genre_list_to_html(article_list, nav));
                        } else {
                            // Format the URL
                            let url;
                            if genre == YomiuriGenreTag::LatestNews {
                                url = format!("{}/{}/{}", *JNAPI_BASE_URL, genre, page.unwrap_or(1));
                            } else {
                                url = format!("{}/{}", *JNAPI_BASE_URL, genre);
                            }

                            // Send the request to the server
                            let request_result = reqwasm::http::Request::get(&url).send().await;
                            let Ok(request_result) = request_result else { 
                                let e = request_result.unwrap_err().to_string();
                                return Err(format!("error fetching from server: {e}"));
                            };

                            // Parse JSON as Vec<JNArticleMeta>
                            let jn_article_meta = request_result.json::<Vec<JNArticleMeta>>().await;
                            let Ok(jn_article_meta) = jn_article_meta else {
                                let e = jn_article_meta.unwrap_err().to_string();
                                return Err(format!("error parsing response as JSON: {e}"));
                            };

                            // Update in-memory cache
                            dispatch_jn.reduce_mut(|yomiuri_map| {
                                let mut hash_map = HashMap::new();
                                hash_map.insert(YomiuriGenreTagSub { genre, subgenre }, jn_article_meta.clone());
                                yomiuri_map.news_lists = Some(hash_map);
                            });

                            // Return final HTML
                            return Ok(yomiuri_genre_list_to_html(&jn_article_meta, nav));
                        }
                    }

                    // Article list not found in in-memory cache. Fetch article list and update in-memory cache.

                    // Format the URL
                    let url;
                    if genre == YomiuriGenreTag::LatestNews {
                        url = format!("{}/{}/{}", *JNAPI_BASE_URL, genre, page.unwrap_or(1));
                    } else {
                        url = format!("{}/{}", *JNAPI_BASE_URL, genre);
                    }

                    // Send the request to the server
                    let request_result = reqwasm::http::Request::get(&url).send().await;
                    let Ok(request_result) = request_result else { 
                        let e = request_result.unwrap_err().to_string();
                        return Err(format!("error fetching from server: {e}"));
                    };
                    
                    // Parse JSON as Vec<JNArticleMeta>
                    let jn_article_meta = request_result.json::<Vec<JNArticleMeta>>().await;
                    let Ok(jn_article_meta) = jn_article_meta else {
                        let e = jn_article_meta.unwrap_err().to_string();
                        return Err(format!("error parsing response as JSON: {e}"));
                    };

                    // Update in-memory cache
                    dispatch_jn.reduce_mut(|yomiuri_map| {
                        if let Some(hash_map) = yomiuri_map.news_lists.as_mut() {
                            hash_map.insert(YomiuriGenreTagSub { genre: genre.clone(), subgenre: subgenre.clone() }, jn_article_meta.clone());
                        } else {
                            let mut hash_map = HashMap::new();
                            hash_map.insert(YomiuriGenreTagSub { genre, subgenre }, jn_article_meta.clone());
                            yomiuri_map.news_lists = Some(hash_map);
                        }
                    });

                    // Return final HTML
                    return Ok(yomiuri_genre_list_to_html(&jn_article_meta, nav));
                }
            }
        })?
    };

    let html_result = result.clone().unwrap();

    Ok(html_result)
}

#[function_component]
pub fn ArticleGenreList(props: &ArticleListProps) -> Html {
    let (_, dispatch) = use_store::<NavbarState>();


    // Set the correct navbar state
    {
        let dispatch = dispatch.clone();
        use_effect_with(props.clone(), move |props| {
            match props.list_metadata {
                ArticleListMetadata::NHK { ref type_ } => {
                    dispatch.set(
                        NavbarState { navbar_state: NavbarViewState::NHK(type_.clone()) }
                    );
                },
                ArticleListMetadata::Yomiuri { ref genre, ref subgenre, ref page } => {
                    dispatch.set( NavbarState { navbar_state: NavbarViewState::Yomiuri(genre.clone()) }
                    );
                },
            }}
        );
    }

    html!{
        <Suspense fallback={html!{<Loading/>}}>
            <main id="main-content">
                <ContentGenre list_metadata={props.list_metadata.clone()}/>
                {
                    match &props.list_metadata {
                        ArticleListMetadata::NHK { type_: _ } => html!{
                            <style>
                            {"@media screen and (max-width: 800px) {
                                .nb-bottom-btn {
                                    & img {
                                    margin-bottom: 100%;
                                    margin-left: 100%;
                                    margin-right: 100%;
                                    }
                                }
                                }"}
                            </style>
                        },
                        ArticleListMetadata::Yomiuri { genre: YomiuriGenreTag::LatestNews, subgenre: _, page: _ } => html!{},
                        ArticleListMetadata::Yomiuri { genre, subgenre, page } => html!{
                            <>
                            <SubgenreButton genre={genre.clone()}/>
                            </>
                        },
                    }
                }
            </main>
        </Suspense>
    }
}