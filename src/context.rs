use std::{collections::HashMap, rc::Rc};

use yew::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NewsCategory {
    Top{page: u8},
    Japan{page: u8},
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NewsProvider {
    NHK,
    Yomiuri
}

#[derive(Clone, Debug, PartialEq)]
pub struct AppCtx {
    pub news_provider: NewsProvider,
    pub active_news: NewsCategory,
}

impl AppCtx {
    pub fn update_active_news_to(self, n: NewsCategory) -> Self {
        Self {
            active_news: n,
            ..self
        }
    }

    pub fn update_news_provider(self, n: NewsProvider) -> Self {
        Self {
            news_provider: n,
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
