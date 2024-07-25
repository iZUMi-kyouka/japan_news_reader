use std::collections::HashMap;

use yew::{prelude::*, suspense::{use_future, use_future_with}};
use yew_router::{hooks::use_navigator, navigator::Navigator};
use yewdux::{use_store, Store};

use crate::{components::navbar::NavbarState, routers::AppRoute, SubGenre, YomiuriGenreTag, JNAPI_BASE_URL};

#[derive(Store, PartialEq, Default, Clone, Debug)]
pub struct YomiuriGenreList {
    hash_map: HashMap<YomiuriGenreTag, Option<Vec<SubGenre>>>
}

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct SubgenreButtonProperties {
    pub genre: YomiuriGenreTag
}

fn nav_to_subgenre(genre: YomiuriGenreTag, subgenre: SubGenre, nav: Navigator) -> Callback<MouseEvent> {
    Callback::from(move |e: MouseEvent| {
        e.stop_propagation();
        e.stop_immediate_propagation();
        nav.push(&AppRoute::YomiuriArticleListSubgenre { genre: genre.clone(), subgenre: subgenre.clone(), page: 1 })
    })
}

#[function_component]
fn Content(props: &SubgenreButtonProperties) -> HtmlResult {
    let nav = use_navigator().unwrap();
    let (state, dispatch) =  use_store::<YomiuriGenreList>();
    let result = {
        let genre = props.genre.clone();
        let (state, dispatch) = (state.clone(), dispatch.clone());
        use_future_with(props.genre.clone(), |_| async move {
            if let Some(Some(subgenre_list)) = state.hash_map.get(&genre) {
                return Ok(subgenre_list.clone())
            } else if let Some(None) = state.hash_map.get(&genre) {
                return Ok(vec![])
            } else {
                let result = reqwasm::http::Request::get(&format!("{}/genre_list", *JNAPI_BASE_URL)).send().await;
                if let Ok(response) = result {
                    let subgenre_list = response.json::<HashMap<YomiuriGenreTag, Option<Vec<SubGenre>>>>().await.unwrap();
                    dispatch.set(YomiuriGenreList { hash_map: subgenre_list.clone() });
                    if let Some(Some(subgenre_list)) = subgenre_list.get(&genre) {
                        return Ok(subgenre_list.clone())
                    } else {
                        return Err("failed to get a list of subgenre from the server");
                    }
                } else {
                    return Err("failed to fetch a list of subgenre from the server");
                }
            }
        })?
    };

    if let Ok(subgenre_list) = result.as_ref() {
        let genre = props.genre.clone();
        return Ok(subgenre_list.iter().map(|s| {
            
            html!{
                <>
                <button class="subgenre-dropdown-item" onclick={nav_to_subgenre(genre.clone(), s.clone(), nav.clone())}>
                    <span class="subgenre-dropdown-item__label">{&s.name}</span>
                </button>
                <script src="./scripts/subgenre-dropdown.js"></script>
                </>
            }
        }).collect::<Html>());
    }

    Ok(html!{"Failed to fetch a list of subgenre from the server"})
}

#[function_component]
pub fn SubgenreButton(props: &SubgenreButtonProperties) -> Html {
    html!{
        <>
        <button id="subgenre-button" class="subgenre-btn">
            <img src="./icons/list.svg" width="32px" height="32px"/>
                <div id="subgenre-dropdown" class="subgenre-dropdown">
                    <Suspense fallback={html!{"Loading..."}}>
                            <Content genre={props.genre.clone()}/>
                    </Suspense>
                </div>
        </button>
        <script src="./scripts/subgenre-dropdown_beforeload.js"></script>
        </>        
    }
}