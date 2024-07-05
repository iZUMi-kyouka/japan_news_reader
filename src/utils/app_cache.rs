use std::collections::HashMap;

use web_sys::HtmlDocument;
use web_sys::wasm_bindgen::JsCast;

pub fn set_cookie<S: AsRef<str>>(s: S) -> Result<(), String> {
    gloo::utils::document().unchecked_into::<HtmlDocument>().set_cookie(s.as_ref())
        .map_err(|e| format!("Failed to set cookie: {:?}", e))?;

    Ok(())
}

pub fn get_all_cookie() -> Option<HashMap<String, String>> {
    let document = gloo::utils::document().unchecked_into::<HtmlDocument>();
    let cookies = document
        .cookie()
        .ok()?;

    let cookies = cookies.split("; ");

    let mut result = HashMap::<String, String>::new();
    for cookie in cookies {
        if cookie.is_empty() {
            continue;
        }

        let mut s = cookie.split('=');
        if let (Some(k), Some(v)) = (s.next(), s.next()) {
            result.insert(k.to_string(), v.to_string());
        }
    }

    Some(result)
}

pub fn get_cookie<S: AsRef<str>>(key: S) -> Option<String> {
    let cookies = get_all_cookie()?;
    Some(cookies.get(key.as_ref())?.to_owned())
}

// pub fn set_cookie<S: ToString>(key: S, value: S) -> Result<(), String> {
//     let cookies = get_all_cookie();
//     if let Some(mut cookies_map) = cookies {
//         _ = cookies_map.insert(key.to_string(), value.to_string());
//         gloo::utils::document().unchecked_into::<HtmlDocument>()
//             .set_cookie(stringify_cookie_map(&cookies_map).as_str())
//             .map_err(|e| format!("Error setting cookie: {:?}", e))?;
//     } else {
//         let mut cookies_map = HashMap::<String, String>::new();
//         _ = cookies_map.insert(key.to_string(), value.to_string());
//         gloo::utils::document().unchecked_into::<HtmlDocument>()
//         .set_cookie(stringify_cookie_map(&cookies_map).as_str())
//         .map_err(|e| format!("Error setting cookie: {:?}", e))?;
//     }
//     Ok(())
// }

// // pub fn remove_cookie<S: AsRef<str>>(key: S) -> Result<(), String> {
    
// // }

// pub fn stringify_cookie_map(map: &HashMap<String, String>) -> String {
//     let mut result = String::new();
//     let kv_iter = map
//         .iter()
//         .map(|(k, v)| {
//             format!("{}={}; ", k, v)
//         })
//         .collect::<String>();
    
//     if kv_iter.len() <= 2 {
//         return "".to_string();
//     }

//     kv_iter[..kv_iter.len()-2].to_string()
// }