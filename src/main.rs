use gloo::console::log;

use gloo_net::{http::Request, Error};
use yew::{function_component, html, use_effect_with, use_state, Callback, Html, UseStateHandle};

use crate::components::{
    card::Card,
    custom_form::{CustomForm, Data},
    fupn::{Fupns, NewFupn},
    loader::Loader,
    main_title::{Color, MainTitle},
    message::Message,
};

mod components;

#[function_component(App)]
pub fn app() -> Html {
    let main_title_load = Callback::from(|message: String| log!(message));

    let custom_form_submit = Callback::from(|data: Data| {
        log!("username is", data.username);
        log!("favorite language is", data.favorite_language);
    });

    //let fupns = use_state(|| vec![]);
    let fupns: UseStateHandle<Option<Fupns>> = use_state(|| None);
    let error: UseStateHandle<Option<Error>> = use_state(|| None);
    {
        let fupns = fupns.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_fupns: Fupns =
                    Request::get("http://127.0.0.1:8080/fupn/api/phone-number?page=1&limit=30")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                fupns.set(Some(fetched_fupns));
            });
            || ()
        });
    }

    let fupn_list_logic = match fupns.as_ref() {
        Some(fupns) => fupns
            .fupns
            .iter()
            .map(|fupn| {
                let new_fupn = NewFupn {
                    fupn: fupn.clone(),
                    img_url: format!(
                        "https://robohash.org/{}@robohash.org?size=100x100&set=set1",
                        fupn.clone().description
                    ),
                };

                html! {
                  <Card new_fupn={new_fupn.clone() }/>
                }
            })
            .collect(),
        None => match error.as_ref() {
            Some(_) => {
                html! {
                    <Message text={"Error getting list of fupns"} css_class={"text-danger"}/>
                }
            }
            None => {
                html! {
                  <Loader />
                }
            }
        },
    };

    html! {
        <div>
            <MainTitle title="Frequently Used Phone Numbers" color={Color::Normal} on_load={main_title_load} />
            <CustomForm onsubmit={custom_form_submit} />
            {fupn_list_logic}
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
