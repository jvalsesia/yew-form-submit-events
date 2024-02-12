use gloo::console::log;
use stylist::yew::styled_component;
use yew::{html, Callback, Html};

use crate::components::{
    custom_form::{CustomForm, Data},
    main_title::{Color, MainTitle},
};

mod components;

#[styled_component(App)]
pub fn app() -> Html {
    let main_title_load = Callback::from(|message: String| log!(message));

    let custom_form_submit = Callback::from(|data: Data| {
        log!("username is", data.username);
        log!("favorite language is", data.favorite_language);
    });

    html! {
        <div>
            <MainTitle title="Hi there!!!!!!!" color={Color::Ok} on_load={main_title_load} />
            <CustomForm onsubmit={custom_form_submit} />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
