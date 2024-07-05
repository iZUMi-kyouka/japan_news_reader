

use std::time::Duration;

use gloo::{console::log, utils::document};
use web_sys::{wasm_bindgen::{closure::Closure, JsCast}, Element};
use yew::{prelude::*, suspense::use_future};
use yewdux::use_store;
use crate::components::loading::Loading;

use crate::{components::main_content::ArticleState, NHKArticlePart, NewsCategory, BASE_URL, NHK_BASE_URL};

#[derive(Clone, PartialEq, Properties)]
pub struct ArticleViewProps {
    pub article_id: String,
    pub article_type: NewsCategory
}

#[function_component]
fn Content(props: &ArticleViewProps) -> HtmlResult {
    let article_id = props.article_id.clone();

    use_effect(|| {
        wasm_bindgen_futures::spawn_local(async move {
            let mut article_wrapper = document().get_element_by_id("article-wrapper");
            while let None = article_wrapper {
                article_wrapper = document().get_element_by_id("article-wrapper");
            gloo::timers::future::sleep(Duration::from_millis(10)).await;

            }

            gloo::timers::future::sleep(std::time::Duration::from_millis(10)).await;
            let article_wrapper = article_wrapper.unwrap();
            article_wrapper.remove_attribute("active");
            article_wrapper.set_attribute("active", "true");
        });

        || {
            log!("article view cleanup ran.");
            let article_wrapper = document().get_element_by_id("article-wrapper");
            if let Some(article_wrapper) = article_wrapper {
                article_wrapper.remove_attribute("active");
                log!("cleanup succesful: removed `active` from article-wrapper");
            } else {
                log!("cleanup failed: article-wrapper is missing.");
            }
        }


    });

    let article_text = {
        let article_type = props.article_type;
        use_future(|| async move {
            let url;
            match article_type {
                NewsCategory::Top { page: _ } => {
                    url = format!("{}{}{}", *BASE_URL, "/api/v1/nhkreader/news/", article_id);
                },
                NewsCategory::Japan { page: _ } => {
                    url = format!("{}{}{}", *BASE_URL, "/api/v1/nhkreader/backstory/", article_id);
                },
            }

            // let show_image = Closure::wrap(Box::new(
            //     |img: Element| {
            //         img.remove();
            //     }
            // ) as Box<dyn Fn(_) -> ()>);
            // let show_image_cb = Some(show_image.as_ref().unchecked_ref());

            let request_result = reqwasm::http::Request::get(&url).send().await;
            let show_img_title = Callback::from(|e: Event| {
                if let Some(img) = e.target() {
                    let img = img.unchecked_into::<Element>();
                    img.remove_attribute("nowloading");
                    let loading_div = img.previous_element_sibling().unwrap();
                    loading_div.remove();
                }
            });
            let show_img_body = Callback::from(|e: Event| {
                if let Some(img) = e.target() {
                    let img = img.unchecked_into::<Element>();
                    img.remove_attribute("nowloading");
                    let div = img.parent_element().unwrap();
                    let loading_div = div.previous_element_sibling().unwrap();
                    loading_div.remove();
                }
            });
    
            let html_result = match request_result {
                Ok(response) => {
                    match article_type {
                        NewsCategory::Top { page: _ } => {
                            let article_json = response.json::<Vec<String>>().await;
                            if let Ok(article_para) = article_json {
                                let html_result = article_para
                                    .iter()
                                    .map(|s| {
                                        html! {
                                        <p class="article__p">{s}</p>
                                        }
                                    })
                                    .collect::<Html>();
                                let html_result = html! {
                                    <>
                                    {html_result}
                                    </>
                                };
                                return Ok(html_result);
                            } else {
                                log!("Error serialising article JSON");
                                return Err(article_json.unwrap_err());
                            };
                        },
                        NewsCategory::Japan { page: _ } => {
                            let article_json = response.json::<Vec<NHKArticlePart>>().await;
                            if let Ok(article_para) = article_json {
                                let html_result = article_para
                                    .iter()
                                    .map(|s| {
                                        match s {
                                            NHKArticlePart::TitleImage { link } => {
                                                html! {
                                                    <div class="article__title-image">
                                                        <div class="img-loading-placeholder">
                                                            <div class="activity"></div>
                                                        </div>
                                                        <img nowloading={"true"} onload={show_img_title.clone()} class="article__title-image" src={format!("{}{}", NHK_BASE_URL, link)}/>
                                                    </div>
                                                }
                                            },
                                            NHKArticlePart::Description(description) => {
                                                html! {
                                                    <p class="article__description">{description}</p>
                                                }
                                            },
                                            NHKArticlePart::BodyImage { link, caption } => {
                                                html! {
                                                    <>
                                                    <figure class="article__body-image">
                                                        <div class="img-loading-placeholder">
                                                            <div class="activity"></div>
                                                        </div>
                                                        <div class="article__body_image">
                                                            <img loading={"lazy"} onload={show_img_body.clone()} src={format!("{}{}", NHK_BASE_URL, link)} nowloading={"true"}/>
                                                        </div>
                                                    </figure>
                                                    <figcaption>{caption}</figcaption>
                                                    </>
                                                }
                                            },
                                            NHKArticlePart::Text(text) => {
                                                html! {
                                                    <p class="article__p">{text}</p>
                                                }
                                            },
                                            _ => {
                                                html!{}
                                            }
                                        }
                                    })
                                    .collect::<Html>();
                                let html_result = html! {
                                    <>
                                    {html_result}
                                    </>
                                };
                                return Ok(html_result);
                            } else {
                                log!("Error serialising article JSON");
                                return Err(article_json.unwrap_err());
                            };
                        }
                    }
                   
                }
                Err(e) => {
                    log!(&format!("Error fetching parsed article from server: {}", e));
                    Err(e)
                }
            };
    
            html_result
        })?
    };

    let article_text = article_text.as_ref().unwrap().clone();
    Ok(article_text)
}

#[function_component]
pub fn ArticleView(props: &ArticleViewProps) -> Html {
    let (state, _) = use_store::<ArticleState>();

    html! {
        <Suspense fallback={html!{<Loading/>}}>
            <article id="article-wrapper" class="article-wrapper">
            <h2 class="article__title">{&state.title}</h2>
            <Content article_id={props.article_id.clone()} article_type={props.article_type.clone()}/>
            </article>
        </Suspense>
    }
}
