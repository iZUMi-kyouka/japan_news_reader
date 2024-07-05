use yew::prelude::*;

#[function_component]
pub fn Loading() -> Html {
    html!(
        <div class="loading-container">
            <img src="./icons/loading.svg" id="loading-spinner"/>
        </div>
    )
}
