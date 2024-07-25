
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use crate::components::navbar::NavbarViewState;
use crate::routers::switch;
use crate::utils::app_cache::{get_all_cookie, set_cookie};
use crate::NHKArticleType;
use crate::{news_scraper::top_news::retrieve_top_news, retrieve_japan_news, routers::AppRoute};
use crate::{AppContext, AppCtx, NHKArticleList, NewsCategory};
use chrono::Utc;
use cookie::CookieBuilder;
use gloo::console::log;
use gloo::utils::document;
use web_sys::Element;
use web_sys::{wasm_bindgen::JsCast, HtmlDocument};
use yew::{prelude::*, suspense::use_future};
use yew_router::prelude::*;
use yewdux::{use_store, Store};
use crate::components::loading::Loading;

use super::navbar::NavbarState;


#[derive(PartialEq, Properties)]
pub struct TopNewsContentProp {
    pub page: u8,
}

#[derive(Default, Clone, PartialEq, Store)]
pub struct ArticleState {
    pub title: String,
}

#[derive(PartialEq, Properties)]
pub struct TopNewsProp {
    pub page: u8,
}

#[derive(PartialEq, Properties)]
pub struct JapanNewsProp {
    pub page: u8,
}

// #[function_component]
// pub fn TopNewsContent(props: &TopNewsContentProp) -> HtmlResult {
//     let (_, dispatch) = use_store::<ArticleState>();
//     let nav = use_navigator().unwrap();
//     let cx = use_context::<AppContext>().unwrap();

//     // Update icon highlight
//     {
//         let page = props.page;
//         let cx = cx.clone();
//         use_effect(move || {
//             log!("use effect ran");
//             cx.dispatch(AppCtx {
//                 active_news: NewsCategory::Top { page },
//                 ..(*cx).clone()
//             });

//             wasm_bindgen_futures::spawn_local(async move {
//                 let element = document().get_element_by_id("main-content");
//                 while let None = element {
//                     gloo::timers::future::sleep(std::time::Duration::from_millis(10)).await;
                    
//                 }

//                 gloo::timers::future::sleep(std::time::Duration::from_millis(10)).await;
//                 let element = element.unwrap();
//                 let _ = element.set_attribute("active", "true");
//             });

//             || {
//                 log!("cleanup ran.");
//                 let main_element = document().get_element_by_id("main-content");
//                 if let Some(main) = main_element {
//                     log!("cleanup succesful: removed `active` from main-content");
//                     let _ = main.remove_attribute("active");
//                 } else {
//                     log!("cleanup failed: main-content is missing.");
//                 }
//             }
//         });
//     }

//     let result = {
//         let page = props.page;
//         let cx = cx.clone();
//         use_future(|| async move {
//              // Check if data in memory and storage are up to date (created within the last 15 minutes)
//             // log!(&format!("context memory: {:#?}", cx.news_metadata.as_ref()));
//             // log!("Future run.");

//             // log!(&format!("cookie expiry: {:?}", &cookie_expiry));
//             // Cache is not yet expired. Try retrieving from memory and local storage
//                 // Check if news article metadata is in memory
//             let result =  get_metadata_from_memory(NewsCategory::Top { page }, cx.clone());
//             if let Ok(news_meta) = result {
//                 // log!("Found cache in memory. Rendering...");
//                 return Ok(news_meta);
//             } else {
//                 // log!(&format!("Cache not found in memory: {}. Finding cache in local storage...", result.unwrap_err()));
//             }

//                 // Check if news article metadata is in local storage
//                 // if let Some(news_meta) = get_metadata(NewsCategory::Top { page }) {
//                 //     // log!("Found cache in local storage. Rendering...");
//                 //     // Cache in memory
//                 //     let result = update_metadata_in_memory(NewsCategory::Top { page }, cx.clone(), news_meta.clone());
//                 //     if let Ok(_) = result  {
//                 //         // log!("Sucessfully loaded news list from local storage to memory.");
//                 //     } else {
//                 //         // log!(&format!("Failed to load news list from local storage to memory: {}", result.unwrap_err()));
//                 //     }
//                 //     return Ok(news_meta);
//                 // } else {
//                 //     // log!("Cache not found in local storage. Fetching news list from server...");
//                 // }

//             // Cache has expired OR 
//             // Cache is unavailable in both memory and local storage. Retrieve and update both cache
//             // Refresh cache expiry
//             let result = retrieve_top_news(page).await;
//             let Ok(news_meta) = result else {
//                 let error = result.unwrap_err();
//                 gloo::console::log!(&error);
//                 return Err(error);
//             };

//             if news_meta.len() == 0 {
//                 gloo::console::log!("Error fetching news list.");
//                 let news_meta = NHKArticleList {
//                     list: news_meta,
//                     last_page: 1
//                 };
//                 return Ok(news_meta);
//             };

//             let news_meta = NHKArticleList {
//                 list: news_meta,
//                 last_page: 1
//             };

//             if let Ok(_) = update_metadata(NewsCategory::Top{ page }, news_meta.clone(), cx) {
//                 let cur_time = (Utc::now().timestamp() + 15*60).to_string();
//                 let cookie = CookieBuilder::new("cacheExpiry", &cur_time)
//                     .path("*")
//                     .max_age(cookie::time::Duration::minutes(15))
//                     .build()
//                     .to_string();
//                 if let Ok(_) = set_cookie(cookie) {
//                     // log!("Successfully updated cache in memory and local storage");
//                     let removed_cookie = CookieBuilder::new("cacheExpiry", "")
//                         .path("*")
//                         .max_age(cookie::time::Duration::milliseconds(0))
//                         .build()
//                         .to_string();
//                     // In 15 minutes, the callback function to delete the cache will be fired.
//                     gloo::timers::callback::Timeout::new(60*15*1000, move || {
//                         let html_document = gloo::utils::document()
//                             .unchecked_into::<HtmlDocument>()
//                             .set_cookie(&removed_cookie);
//                         if let Err(e) = html_document {
//                             // log!(&format!("failed to remove cookie: {:?}", e));
//                         }
//                     }).forget();
//                 }
//             }

//             // gloo::console::log!(format!("{:?}", &news_meta));
//             Ok(news_meta)
//         })?
//     };

//     let result = result.clone().unwrap();

//     let html_result = result.list.into_iter()
//         .map(|news_meta| {
//             let view_article = {
//                 let nav = nav.clone();
//                 let dispatch = dispatch.clone();
//                 let article_title = news_meta.title.clone().unwrap_or("<TITLE UNAVAILABLE>".to_string());
//                 let news_id = news_meta.relative_url.as_str().split('/').skip(4).next().map(|s| s.to_string()).unwrap_or_default();
//                 Callback::from(move |_: MouseEvent| {
//                     dispatch.set(ArticleState { title: article_title.clone() });
//                     nav.push(&AppRoute::TopNewsArticle { article_id: String::from(&news_id),  })
//                 })
//             };
//             html!{
//                 <article class="news-article flex-col cursor-ptr" onclick={view_article}>
//                     <span class="news-article__title link-deco" tabindex=0>{news_meta.title.as_ref().map(|s| s.as_str()).unwrap_or("<TITLE UNAVAILABLE>")}</span>
//                     <span class="news-article__date">{news_meta.date.as_ref().map(|d| {
//                         d.format("%d %B %Y").to_string()
//                     }).unwrap_or("".to_string())}</span>
//                 </article>
//             }
//     })
//     .collect::<Html>();

//     Ok(html_result)
// }

// #[function_component]
// pub fn JapanNewsContent(props: &JapanNewsProp) -> HtmlResult {
//     let nav = use_navigator().unwrap();

//     // Update icon highlight, apply animation
//     {
//         let page = props.page;
//         use_effect(move || {
//             log!("use effect ran.");

//             wasm_bindgen_futures::spawn_local(async move {
//                 let element = document().get_element_by_id("main-content");
//                 while let None = element {
//                     gloo::timers::future::sleep(std::time::Duration::from_millis(10)).await;
//                 }

//                 gloo::timers::future::sleep(Duration::from_millis(10)).await;
//                 let element = element.unwrap();
//                 let _ = element.set_attribute("active", "true");
//             });

//             || {
//                 log!("cleanup ran.");
//                 let main_element = document().get_element_by_id("main-content");
//                 if let Some(main) = main_element {
//                     main.remove_attribute("active");
//                     log!("cleanup succesful: removed `active` from main-content");
//                 } else {
//                     log!("cleanup failed: main-content is missing.");
//                 }
//             }
//         });
//     }

//     let result = {
//         let page = props.page;
//         use_future(|| async move {
//             // log!(&format!("Context memory: {:#?}", cx.news_metadata.as_ref()));
//             // Check if data in memory and storage are up to date (created within the last 15 minutes)
//             let mut cookie_expiry = None;
//             if let Some(cookie) = get_all_cookie() {
//                 cookie_expiry = cookie.get("cacheExpiry").map(|s| s.to_owned());
//             }
    
//             let cur_time = Utc::now().timestamp();

//             // Cache is not yet expired. Try retrieving from memory and local storage
//             // Check if news article metadata is in memory
//             if let Ok(news_meta) = get_metadata_from_memory(NewsCategory::Japan { page }, cx.clone()) {
//                 // log!("Found cache in memory. Rendering...");
//                 return Ok(news_meta);
//                 }
    
//                 // Check if news article metadata is in local storage
//                 // if let Some(news_meta) = get_metadata(NewsCategory::Japan  { page }) {
//                 //     // Cache in memory
//                 //     if let Ok(_) = update_metadata_in_memory(NewsCategory::Japan { page }, cx.clone(), news_meta.clone()) {
//                 //         // log!("Found cache in local storage. Loading to memory...");
//                 //     }
//                 //     return Ok(news_meta);
//                 // }
    
//             // Cache has expired OR 
//             // Cache is unavailable in both memory and local storage. Retrieve and update both cache
//             // Refresh cache expiry
//             let result = retrieve_japan_news(page).await;
//             let Ok(news_meta) = result else {
//                 let error = result.unwrap_err();
//                 gloo::console::log!(&error);
//                 return Err(error);
//             };
    
//             if let Ok(_) = update_metadata(NewsCategory::Japan { page }, news_meta.clone(), cx) {
//                 let cur_time = (Utc::now().timestamp() + 15*60).to_string();
//                 let cookie = CookieBuilder::new("cacheExpiry", &cur_time)
//                     .path("*")
//                     .max_age(cookie::time::Duration::minutes(15))
//                     .build()
//                     .to_string();
//                 if let Ok(_) = set_cookie(cookie) {
//                     // log!("Successfully updated cache in memory and local storage");
//                     let removed_cookie = CookieBuilder::new("cacheExpiry", "")
//                         .path("*")
//                         .max_age(cookie::time::Duration::milliseconds(0))
//                         .build()
//                         .to_string();
//                 }
//             }
    
//             gloo::console::log!(format!("{:?}", &news_meta));
//             Ok(news_meta)
//         })?  
//     };

//     let result = result.as_ref().unwrap();

//     // Build the HTML
//     let html_result = result.list.iter()
//         .map(|news_meta| {
//             let view_article = {
//                 let nav = nav.clone();
//                 let article_title = news_meta.title.clone().unwrap_or("<TITLE UNAVAILABLE>".to_string());
//                 let news_id = news_meta.relative_url.as_str().split('/').skip(5).next().map(|s| s.to_string()).unwrap_or_default();
//                 Callback::from(move |_: MouseEvent| {
//                     nav.push(&AppRoute::JapanNewsArticle { article_id: String::from(&news_id), })
//                 })
//             };
//             html!{
//             <article class="news-article flex-col" onclick={view_article}>
//                 <span class="news-article__title link-deco kb-foc" tabindex=0 >{news_meta.title.as_ref().map(|s| s.as_str()).unwrap_or("<TITLE UNAVAILABLE>")}</span>
//                 <span class="news-article__date">{news_meta.date.as_ref().map(|d|                         
//                     d.format("%d %B %Y").to_string()).unwrap_or("".to_string())}</span>
//             </article>
//             }
//         })
//         .collect::<Html>();

//     Ok(html_result)
// }

// #[function_component]
// pub fn TopNews(props: &TopNewsProp) -> Html {
//     let (_, dispatch) = use_store::<NavbarState>();
//     use_effect(move || {
//         dispatch.set(
//             NavbarState {
//                 navbar_state: NavbarViewState::NHK(NHKArticleType::News)
//             }
//         )
//     });

//     html! {
//         <Suspense fallback={html!{<Loading/>}}>
//             <main id="main-content">
//                 <TopNewsContent page={props.page}/>
//             </main>
//         </Suspense>
//     }
// }

// #[function_component]
// pub fn JapanNews(props: &JapanNewsProp) -> Html {
//     let (_, dispatch) = use_store::<NavbarState>();
//     use_effect(move || {
//         dispatch.set(
//             NavbarState {
//                 navbar_state: NavbarViewState::NHK(NHKArticleType::Backstory)
//             }
//         )
//     });

//     html! {
//         <Suspense fallback={html!{<img src="./icons/loading.svg" id="loading-spinner"/>}}>
//             <main id="main-content">
//                 <JapanNewsContent page={props.page}/>
//             </main>
//         </Suspense>
//     }
// }

#[function_component]
pub fn MainContent() -> Html {
    html! {
        <>
            <Switch<AppRoute> render={switch}/>
        </>
    }
}
