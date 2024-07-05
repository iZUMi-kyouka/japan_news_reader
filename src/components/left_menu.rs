use gloo::utils::document;
use yew::prelude::*;

#[function_component]
pub fn LeftMenu() -> Html {

    let close_btn = {
        Callback::from(|_: MouseEvent| {
            let Some(left_menu) = document().get_element_by_id("left-menu") else { return () };
            let _ = left_menu.remove_attribute("active");
        })
    };

    html!{
        <div id="left-menu">
            <button id="left-menu-close-btn" class="nb-btn" onclick={close_btn}><img class="ui-icon" src="./icons/close.svg" width="40px" height="40px"/></button>
            <div id="left-menu__news-provider">
                <span>{"Select news provider"}</span>
                <button id="nhk-logo-left-menu" class="nb-btn"><img src="./icons/nhk.svg"/></button>
                <button id="yomiuri-logo-left-menu" class="nb-btn"><img src="./icons/yomiuri.svg"/></button>
            </div>
        </div>
    }
}