use gloo::utils::document;
use yew::prelude::*;

pub fn turn_off_all_navbar_bottom_buttons() {
    let btns = document().get_elements_by_class_name("nb-bottom-btn");
    for i in 0..btns.length() {
        if let Some(e) = btns.get_with_index(i) {
            let _ = e.remove_attribute("active");
        };
    }
}