use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NHKArticleType {
    News,
    Backstory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NHKArticleMeta {
    pub title: Option<String>,
    pub relative_url: String,
    pub date: Option<NaiveDate>,
    pub type_: NHKArticleType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NHKArticleList {
    pub list: Vec<NHKArticleMeta>,
    pub last_page: u8
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NHKArticlePart {
    Text (String),
    Title (String),
    Description (String),
    TitleImage {
        link: String,
    },
    BodyImage {
        link: String,
        caption: String
    }
}