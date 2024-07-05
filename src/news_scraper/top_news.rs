use gloo::console::log;

use crate::{model::*, BASE_URL};

pub async fn retrieve_top_news(p: u8) -> Result<Vec<NHKArticleMeta>, String> {
    log!("fetching top news...");
    let url = format!("{}{}{}", *BASE_URL, "/api/v1/nhkreader/top/", p);
    // Retrieve TOP news front page

    let html_news_list = reqwasm::http::Request::get(&url)
        .send()
        .await
        .map_err(|e| format!("network error: {:?}", e))?
        .json::<Vec<NHKArticleMeta>>()
        .await
        .map_err(|e| format!("html text parsing error: {:?}", e))?;

    Ok(html_news_list)
}
