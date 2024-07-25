

use std::time::Duration;

use gloo::{console::log, utils::{document, window}};
use web_sys::{wasm_bindgen::{closure::Closure, JsCast}, Element};
use yew::{prelude::*, suspense::{use_future, use_future_with}};
use yewdux::{dispatch, use_store};
use crate::{components::{loading::Loading, navbar::{NavbarState, NavbarViewState}}, main, AppContext, ArticleMetadata, JNArticle, NHKArticleType, NewsProvider, NHK_BASE_URL};

use crate::{components::main_content::ArticleState, NHKArticlePart, NewsCategory, NHKAPI_BASE_URL};

#[derive(Clone, PartialEq, Properties)]
pub struct ArticleViewProps {
    pub article_metadata: ArticleMetadata
}

#[function_component]
fn Content(props: &ArticleViewProps) -> HtmlResult {
    let cx = use_context::<AppContext>().unwrap();

    {
        use_effect(move || {
            wasm_bindgen_futures::spawn_local(async move {
                let mut article_wrapper = document().get_element_by_id("article-wrapper");
                while let None = article_wrapper {
                    article_wrapper = document().get_element_by_id("article-wrapper");
                gloo::timers::future::sleep(Duration::from_millis(10)).await;
    
                }
    
                gloo::timers::future::sleep(std::time::Duration::from_millis(10)).await;
                let article_wrapper = article_wrapper.unwrap();
                let _ = article_wrapper.remove_attribute("active");
                let _ = article_wrapper.set_attribute("active", "true");
            });

            window().scroll_to_with_x_and_y(0f64, 0f64);
    
            || {
                log!("article view cleanup ran.");
                let article_wrapper = document().get_element_by_id("article-wrapper");
                if let Some(article_wrapper) = article_wrapper {
                    let _ = article_wrapper.remove_attribute("active");
                    log!("cleanup succesful: removed `active` from article-wrapper");
                } else {
                    log!("cleanup failed: article-wrapper is missing.");
                }
            }
        });
    }
   

    let article_text = {
        let cx = cx.clone();
        let url = props.article_metadata.to_link();
        use_future_with((props.article_metadata.clone()), |article_metadata| async move {
            let request_result = reqwasm::http::Request::get(&url).send().await;

            let show_img_title = Callback::from(|e: Event| {
                if let Some(img) = e.target() {
                    let img = img.unchecked_into::<Element>();
                    let _ = img.remove_attribute("nowloading");
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
            
            match article_metadata.as_ref() {
                &ArticleMetadata::NHK { ref type_, ref article_id } => {
                    // Set navbar state

                    if cx.news_provider != NewsProvider::NHK {
                        cx.dispatch((*cx).clone().update_news_provider(NewsProvider::NHK));
                    }
                    let html_result = match request_result {
                        Ok(response) => {
                            match type_ {
                                NHKArticleType::News => {
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
                                NHKArticleType::Backstory => {
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
                                            <h2 class="article__title">{
                                                if let NHKArticlePart::Title(t) = article_para.iter().skip(1).next().unwrap() {
                                                    html!{t}
                                                } else {
                                                    html!{}
                                                }
                                            }</h2>
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
                },
                &ArticleMetadata::Yomiuri { ref genre, ref subgenre, ref article_id } => {
                    if cx.news_provider != NewsProvider::Yomiuri {
                        cx.dispatch((*cx).clone().update_news_provider(NewsProvider::Yomiuri));
                    }
                    let Ok(response) = request_result else {
                        let e = request_result.unwrap_err();
                        log!(&format!("Error fetching parsed article from server: {}", e));
                        return Err(e); 
                    };

                    let jn_article_result = response.json::<JNArticle>().await;
                    let Ok(jn_article) = jn_article_result else {
                        let e = jn_article_result.unwrap_err();
                        log!("Error serialising article JSON");
                        return Err(e);
                    };


                    Ok(html!{
                        <>
                        <h2 class="article__title">{jn_article.title.unwrap_or_default()}</h2>
                        {
                            if let Some(url) = jn_article.img_url {
                                html! {
                                    <div class="article__title-image">
                                    <div class="img-loading-placeholder">
                                        <div class="activity"></div>
                                    </div>
                                    <img nowloading="true" onload={show_img_title.clone()} class="article__title-image" src={url}/>
                                </div>
                                }
                            } else {
                                html!{}
                            }
                        }


                        <div class="article__writer">{jn_article.writers.unwrap_or_default()}</div>
                        <div class="article__dt">{jn_article.publication_date.unwrap_or_default()}</div>
                        { jn_article.text.iter().map(|s| html!{<p class="article__p">{s}</p>}).collect::<Html>() }
                        </>
                    })


                }
            }
        })?
    };
       

    let article_text = article_text.as_ref().unwrap().clone();
    Ok(article_text)
}

#[function_component]
pub fn ArticleView(props: &ArticleViewProps) -> Html {
    let (state, dispatch) = use_store::<NavbarState>();

    {
        let article_view_props = props.article_metadata.clone();
        let (state, dispatch) = (state.clone(), dispatch.clone());
        use_effect(move || {
            match article_view_props {
                ArticleMetadata::NHK { type_, article_id } => {
                    dispatch.set(NavbarState { navbar_state: NavbarViewState::NHK(type_.clone()) });
                },
                ArticleMetadata::Yomiuri { genre, subgenre, article_id } => {
                    dispatch.set(NavbarState { navbar_state: NavbarViewState::Yomiuri(genre.clone()) });
                },
            }
        });
    }


    html! {
        <Suspense fallback={html!{<Loading/>}}>
            <article id="article-wrapper" class="article-wrapper">
                <Content article_metadata={props.article_metadata.clone()} />
            </article>
        </Suspense>
    }
}
