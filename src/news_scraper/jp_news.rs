use gloo::console::log;

use crate::{model::*, BASE_URL};

pub async fn retrieve_japan_news(page: u8) -> Result<NHKArticleList, String> {
    log!("fetching japan news...");
    let url = format!("{}{}{}", *BASE_URL, "/api/v1/nhkreader/japan/", page);
    // Retrieve TOP news front page

    let html_news_list = reqwasm::http::Request::get(&url)
        .send()
        .await
        .map_err(|e| format!("network error: {:?}", e))?
        .json::<NHKArticleList>()
        .await
        .map_err(|e| format!("html text parsing error: {:?}", e))?;

    Ok(html_news_list)
}
