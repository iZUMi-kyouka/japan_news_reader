use std::{fmt::Display, str::FromStr};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use yew::Properties;

use crate::{JNAPI_BASE_URL, NHKAPI_BASE_URL};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ArticleListMetadata {
    NHK {
        type_: NHKArticleType,
    },
    Yomiuri {
        genre: YomiuriGenreTag,
        subgenre: Option<SubGenre>,
        page: Option<u8>
    },
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ArticleMetadata {
    NHK {
        type_: NHKArticleType,
        article_id: String
    },
    Yomiuri {
        genre: YomiuriGenreTag,
        subgenre: SubGenre,
        article_id: String
    },
}

impl ArticleMetadata {
    pub fn to_link(&self) -> String {
        match self {
            ArticleMetadata::NHK { type_, article_id } => {
                match type_ {
                    NHKArticleType::News => {
                        format!("{}/news/{}", *NHKAPI_BASE_URL, article_id)
                    },
                    NHKArticleType::Backstory => {
                        format!("{}/backstories/{}", *NHKAPI_BASE_URL, article_id)
                    },
                }
            },
            ArticleMetadata::Yomiuri { genre, subgenre, article_id } => {
                format!("{}/article/{}/{}/{}", *JNAPI_BASE_URL, genre.as_str(), subgenre.link.as_str(), article_id)
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NHKArticleType {
    News,
    Backstory,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum YomiuriGenreTag {
    LatestNews,
    Politics,
    Society,
    Business,
    World,
    NewsServices,
    EditorialColumns,
    Sports,
    ScienceNature,
    Culture,
    JNSpecialties,
    Features,
}

impl YomiuriGenreTag {
    pub fn as_str(&self) -> &'static str {
        match self {
            YomiuriGenreTag::LatestNews => "latestnews",
            YomiuriGenreTag::Politics => "politics",
            YomiuriGenreTag::Society => "society",
            YomiuriGenreTag::Business => "business",
            YomiuriGenreTag::World => "world",
            YomiuriGenreTag::NewsServices => "news-services",
            YomiuriGenreTag::EditorialColumns => "editorial",
            YomiuriGenreTag::Sports => "sports",
            YomiuriGenreTag::ScienceNature => "science-nature",
            YomiuriGenreTag::Culture => "culture",
            YomiuriGenreTag::JNSpecialties => "original",
            YomiuriGenreTag::Features => "features",
        }
    }

    pub fn as_str_pretty(&self) -> String {
        format!("{}{}", &self.as_str().chars().next().map(|s| s.to_ascii_uppercase()).unwrap(), &self.as_str()[1..])
    }
}

impl Display for YomiuriGenreTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for YomiuriGenreTag {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = match s {
            "latestnews"  => Self::LatestNews,
            "politics" => Self::Politics,
            "society"  => Self::Society,
            "business"  => Self::Business,
            "world" => Self::World,
            "news-services" => Self::NewsServices,
            "editorial" => Self::EditorialColumns,
            "sports"  => Self::Sports,
            "science-nature" => Self::ScienceNature,
            "culture" => Self::Culture,
            "original" => Self::JNSpecialties,
            "features" => Self::Features,
            _ => {
                return Err(());
            }
        };

        Ok(result)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct JNArticleMeta {
    pub img_url: Option<String>,
    pub title: Option<String>,
    pub url: String,
    pub date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JNArticle {
    pub title: Option<String>,
    pub img_url: Option<String>,
    pub writers: Option<String>,
    pub publication_date: Option<String>,
    pub text: Vec<String>
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SubGenre{
    pub name: String,
    pub link: String
}

impl Display for SubGenre {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.link.as_str());
        Ok(())
    }
}

impl FromStr for SubGenre {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "manga-anime" => Ok(SubGenre { name: "Manga & Anime".to_string(), link: s.to_string()}),
            "film-tv" => Ok(SubGenre { name: "Film & TV".to_string(), link: s.to_string()}),
            "kabuki-noh-rakugo" => Ok(SubGenre { name: "Kabuki / Noh / Rakugo".to_string(), link: s.to_string()}),
            "books-literature" => Ok(SubGenre { name: "Books & Literature".to_string(), link: s.to_string()}),
            "japan-focusd" => Ok(SubGenre { name: "Japan in Focus".to_string(), link: s.to_string()}),
            "cartoon" => Ok(SubGenre { name: "Political Cartoons".to_string(), link: s.to_string()}),
            "crime-courts" => Ok(SubGenre { name: "Crime & Courts".to_string(), link: s.to_string()}),
            "donald-keenes-legacy" => Ok(SubGenre{ name: "Donald Keenes' Legacy".to_string(), link: s.to_string()}),
            "ramen-japan" => Ok(SubGenre { name: "Ramen of Japan".to_string(), link: s.to_string()}),
            "delicious-japan" => Ok(SubGenre { name: "Delicious Beautiful Spectacular JAPAN".to_string(), link: s.to_string()}),
            "politics-government" => Ok(SubGenre { name: "Politics & Government".to_string(), link: s.to_string()}),
            "defense-security" => Ok(SubGenre { name: "Defense & Security".to_string(), link: s.to_string()}),
            "asia-pacific" => Ok(SubGenre { name: "Asia - Pacific".to_string(), link: s.to_string()}),
            "us-canada" => Ok(SubGenre { name: "US & Canada".to_string(), link: s.to_string()}),
            "insights-world" => Ok(SubGenre { name: "Insights into the World".to_string(), link: s.to_string()}),
            "washingtonpost" => Ok(SubGenre { name: "Washington Post".to_string(), link: s.to_string()}),
            "ap" => Ok(SubGenre { name: "Associated Press".to_string(), link: s.to_string()}),
            "afp-jiji" => Ok(SubGenre { name: "AFP - Jiji".to_string(), link: s.to_string()}),
            "soccer-worldcup" => Ok(SubGenre { name: "Soccer World Cup".to_string(), link: s.to_string()}),
            _ => {
                let mut token = s.split('-').map(|s| s.to_string()).collect::<Vec<_>>();
                let first_word = token.get_mut(0).unwrap();
                let capitalised = format!("{}{}", first_word.as_str().chars().next().map(|s| s.to_ascii_uppercase()).unwrap(), &first_word[1..]);
                first_word.clear();
                first_word.push_str(&capitalised);
                Ok(SubGenre { name: token.join(" "), link: s.to_string() })
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct YomiuriGenre {
    pub genre: YomiuriGenreTag,
    pub subgenre: Option<Vec<SubGenre>>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct YomiuriGenreTagSub {
    pub genre: YomiuriGenreTag,
    pub subgenre: Option<SubGenre>
}