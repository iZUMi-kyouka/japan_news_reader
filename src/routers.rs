
use crate::components::article_view::ArticleView;
use crate::components::loading::Loading;
use crate::components::main_content::{JapanNews, TopNews};
use crate::NewsCategory;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    HomeDefault,
    #[at("/:page")]
    Home { page: u8 },
    #[at("/japan/:page")]
    Japan { page: u8 },
    #[at("/news/:article_id")]
    TopNewsArticle { article_id: String },
    #[at("/backstory/:article_id")]
    JapanNewsArticle { article_id: String },
    #[at("/dev/loading")]
    Loading,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::HomeDefault => html! {<TopNews page={1}/>},
        AppRoute::Home { page } => html! {<TopNews {page}/>},
        AppRoute::Japan { page }=> html! {<JapanNews {page}/>},
        AppRoute::TopNewsArticle { article_id } => html! {<ArticleView {article_id} article_type={NewsCategory::Top { page: 0 }}/>},
        AppRoute::JapanNewsArticle { article_id } => html! {<ArticleView {article_id} article_type={NewsCategory::Japan { page: 0 }}/>},
        AppRoute::NotFound => html! {{"Not Found."}},
        AppRoute::Loading => html! {<Loading/>},
    }
}
