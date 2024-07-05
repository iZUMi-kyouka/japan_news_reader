use std::{collections::HashMap, rc::Rc};

use yew::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NewsCategory {
    Top{page: u8},
    Japan{page: u8},
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NewsView {
    NHK,
    Yomiuiri
}

#[derive(Clone, Debug, PartialEq)]
pub struct AppCtx {
    pub news_view: NewsView,
    pub active_news: NewsCategory,
    pub news_metadata: Option<HashMap<String, String>>
}

impl AppCtx {
    pub fn update_active_news_to(self, n: NewsCategory) -> Self {
        Self {
            active_news: n,
            ..self
        }
    }

    pub fn update_news_metadata_to(self, news_metadata: Option<HashMap<String, String>>) -> Self {
        Self {
            news_metadata,
            ..self
        }
    }
}

pub type AppContext = UseReducerHandle<AppCtx>;

impl Reducible for AppCtx {
    type Action = AppCtx;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        action.into()
    }
}
