
use gloo::storage::Storage;

use crate::{AppContext, NHKArticleList, NewsCategory, NewsMetaMap};


// pub fn get_metadata_from_memory(n: NewsCategory, cx: AppContext) -> Result<NHKArticleList, String> {
//     let news_metadata = cx.news_metadata.as_ref().ok_or("News map is unavailable in memory".to_string())?;
//     match n {
//         NewsCategory::Japan { page } => {
//             Ok(
//                 serde_json::from_str::<NHKArticleList>(
//                     news_metadata.get(&format!("JapanNews-{}", page)).ok_or("News map in memory does not contain Japan news".to_string())?.as_str()
//                 ).map_err(|e| format!("Error serializing NHKArticleList from memory: {}", e))?
//             )

//         },
//         NewsCategory::Top { page } => {
//             Ok(
//                 serde_json::from_str::<NHKArticleList>(
//                     news_metadata.get(&format!("TopNews-{}", page)).ok_or("News map in memory does not contain Top news".to_string())?.as_str()
//                 ).map_err(|e| format!("Error serializing NHKArticleList from memory: {}", e))?
//             )
//         }
//     }
// }

// pub fn update_metadata_in_memory(n: NewsCategory, cx: AppContext, new_data: NHKArticleList) -> Result<(), String> {
//     let mut news_metadata = cx.news_metadata.clone();
//     let new_data_raw = serde_json::to_string(&new_data).map_err(|e| format!("Failed to serialize new data: {}", e))?;
//     let mut new_map = NewsMetaMap::new();

//     match n {
//         NewsCategory::Japan { page } => {
//             if let Some(ref mut news_metadata) = news_metadata {
//                 news_metadata.insert(format!("JapanNews-{}", page), new_data_raw);
//                 cx.dispatch((*cx).clone().update_news_metadata_to(Some(news_metadata.clone())));
//             } else {
//                 new_map.insert(format!("JapanNews-{}", page), new_data_raw);
//                 cx.dispatch((*cx).clone().update_news_metadata_to(Some(new_map)));
//             }
//         },
//         NewsCategory::Top { page } => {
//             if let Some(ref mut news_metadata) = news_metadata {
//                 news_metadata.insert(format!("TopNews-{}", page), new_data_raw);
//                 cx.dispatch((*cx).clone().update_news_metadata_to(Some(news_metadata.clone())));
//             } else {
//                 new_map.insert(format!("TopNews-{}", page), new_data_raw);
//                 cx.dispatch((*cx).clone().update_news_metadata_to(Some(new_map)));
//             }

//         }
//     }


//     Ok(())
// }


pub fn get_metadata(n: NewsCategory) -> Option<NHKArticleList> {
    let news_metadata = gloo::storage::LocalStorage::get::<String>("NHKReader-NewsMetaHashMap").ok()?;
    let news_metadata = serde_json::from_str::<NewsMetaMap>(&news_metadata).ok()?;

    match n {
        NewsCategory::Japan { page } => {
            let news_meta_one_page = news_metadata.get(&format!("JapanNews-{}", page))?;
            let news_meta = serde_json::from_str::<NHKArticleList>(&news_meta_one_page).ok()?;
            return Some(news_meta);
        },
        NewsCategory::Top { page } => {
            let news_meta_one_page = news_metadata.get(&format!("TopNews-{}", page))?;
            let news_meta = serde_json::from_str::<NHKArticleList>(&news_meta_one_page).ok()?;
            return Some(news_meta);
        }
    }
}

// Update metadata both in memory and local storage
// pub fn update_metadata(n: NewsCategory, new_data: NHKArticleList, cx: AppContext) -> Result<(), String> {
//     let new_data_raw = serde_json::to_string(&new_data).map_err(|e| format!("Failed to serialise given data: {}", e))?;
//     let existing_map = gloo::storage::LocalStorage::get::<String>("NHKReader-NewsMetaHashMap");

//     if let Ok(existing_map) = existing_map {
//         let mut existing_map = serde_json::from_str::<NewsMetaMap>(&existing_map).map_err(|e| format!("Failed to serialise existing map: {}", e))?;
//         match n {
//             NewsCategory::Japan { page } => {
//                 _ = existing_map.insert(format!("JapanNews-{}", page), new_data_raw);
//             }
//             NewsCategory::Top { page } => {
//                 _ = existing_map.insert(format!("TopNews-{}", page), new_data_raw);
//             }
//         }
//         let updated_map_raw = serde_json::to_string(&existing_map).map_err(|e| format!("Failed to serialise updated map: {}", e))?;
//         cx.dispatch((*cx).clone().update_news_metadata_to(Some(existing_map.clone())));
//         gloo::storage::LocalStorage::set("NHKReader-NewsMetaHashMap", updated_map_raw).map_err(|e| format!("Failed to update local storage: {}", e))?;
//     } else {
//         let mut new_map = NewsMetaMap::new();
//         match n {
//             NewsCategory::Japan { page }=> {
//                 _ = new_map.insert(format!("JapanNews-{}", page), new_data_raw);
//             }
//             NewsCategory::Top { page } => {
//                 _ = new_map.insert(format!("TopNews-{}", page), new_data_raw);
//             }
//         }
//         let updated_map_raw = serde_json::to_string(&new_map).map_err(|e| format!("Failed to serialise updated map: {}", e))?;
//         cx.dispatch((*cx).clone().update_news_metadata_to(Some(new_map.clone())));
//         gloo::storage::LocalStorage::set("NHKReader-NewsMetaHashMap", updated_map_raw).map_err(|e| format!("Failed to update local storage: {}", e))?;
//     }

//     Ok(())
// }