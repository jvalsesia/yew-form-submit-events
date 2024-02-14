use yew::prelude::*;

use super::fupn::NewFupn;

#[derive(Properties, PartialEq)]
pub struct CardProp {
    pub new_fupn: NewFupn,
}

#[function_component(Card)]
pub fn card(CardProp { new_fupn }: &CardProp) -> Html {
    html! {
    <div class="card">
    <img src={new_fupn.img_url.clone()} alt="img" style="width:50%"/>
        <div class="container">
            <p class="fw-bold mb-1">{format!("Id: {}", new_fupn.fupn.id.clone())}</p>
            <p class="fw-normal mb-1">{format!("Phone Number: {}",new_fupn.fupn.phone_number.clone())}</p>
            <p class="fw-normal mb-1">{format!("Description: {}",new_fupn.fupn.description.clone().to_string())}</p>
            <p class="fw-normal mb-1">{format!("Cration Date: {}",new_fupn.fupn.creation_date.clone().to_string())}</p>
            <p class="fw-normal mb-1">{format!("Update Date: {}",new_fupn.fupn.update_date.clone().to_string())}</p>
        </div>
        <p></p>
    </div>

    }
}
