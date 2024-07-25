
use crate::components::article_view::ArticleView;
use crate::components::loading::Loading;
// use crate::components::main_content::{JapanNews, TopNews};
use crate::{ArticleListMetadata, ArticleMetadata, SubGenre, YomiuriGenreTag};
use yew::prelude::*;
use yew_router::prelude::*;
use crate::NHKArticleType;
use crate::components::article_list::*;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    DefaultView,
    #[at("/news")]
    NHKHome,
    #[at("/japan/:page")]
    Japan { page: u8 },
    #[at("/nhk/news/:article_id")]
    TopNewsArticle { article_id: String },
    #[at("/nhk/backstory/:article_id")]
    JapanNewsArticle { article_id: String },
    #[at("/yomiuri/latestnews")]
    YomiuriLatestNews,
    #[at("/yomiuri/:genre/:subgenre/:page")]
    YomiuriArticleListSubgenre { genre: YomiuriGenreTag, subgenre: SubGenre, page: u8 },
    #[at("/yomiuri/:genre")]
    YomiuriArticleListGenre { genre: YomiuriGenreTag },
    #[at("/yomiuri/article/:genre/:subgenre/:article_id")]
    YomiuriArticle { genre: YomiuriGenreTag, subgenre: String,  article_id: String},
    #[at("/dev/loading")]
    Loading,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::DefaultView => html! {<ArticleGenreList list_metadata={ArticleListMetadata::NHK { type_: NHKArticleType::News }}/>},
        AppRoute::NHKHome => html! {<ArticleGenreList list_metadata={ArticleListMetadata::NHK { type_: NHKArticleType::News }}/>},
        AppRoute::Japan { page }=> html! {<ArticleGenreList list_metadata={ArticleListMetadata::NHK { type_: NHKArticleType::Backstory }}/>},
        AppRoute::TopNewsArticle { article_id } => html! {<ArticleView article_metadata={ArticleMetadata::NHK { type_: NHKArticleType::News, article_id }}/>},
        AppRoute::JapanNewsArticle { article_id } => html! {<ArticleView article_metadata={ArticleMetadata::NHK { type_: NHKArticleType::Backstory, article_id }}/>},
        AppRoute::YomiuriLatestNews => html!{<ArticleGenreList list_metadata={ArticleListMetadata::Yomiuri { genre: YomiuriGenreTag::LatestNews, subgenre: None, page: Some(1) }}/>},
        AppRoute::YomiuriArticleListGenre { genre } => html!{<ArticleGenreList list_metadata={ArticleListMetadata::Yomiuri { genre, subgenre: None, page: None }}/>},
        AppRoute::YomiuriArticleListSubgenre { genre, subgenre, page } => html!{
            <ArticleSubGenreList list_metadata={ArticleListMetadata::Yomiuri { genre, subgenre: Some(subgenre), page: Some(page) }}/>},
        AppRoute::YomiuriArticle { genre, subgenre, article_id } => html!{
            <ArticleView article_metadata={ArticleMetadata::Yomiuri { genre, subgenre: SubGenre { name: subgenre.clone(), link: subgenre  }, article_id }} />},
        AppRoute::NotFound => html! {{"Not Found."}},
        AppRoute::Loading => html! {<Loading/>},
    }
}
