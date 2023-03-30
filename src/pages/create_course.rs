use ycl::{
    elements::{
        icon::BBIconType,
        input::{BBInput, BBInputType},
        text_area::BBTextArea,
        title::{BBTitle, BBTitleLevel},
    },
    foundations::{
        align_text::AlignText,
        container::{BBContainer, BBContainerMargin},
    },
};
use yew::{function_component, html, use_effect, Html};
use yew_router::prelude::use_navigator;
use yewdux::prelude::use_store;

use crate::{
    router::Routes,
    stores::{
        alerts::{AlertsStore, AlertsStoreBuilder},
        auth_store::AuthStore,
    },
};

#[function_component(CreateCourse)]
pub fn component() -> Html {
    let (auth_store, _) = use_store::<AuthStore>();
    let navigator = use_navigator().unwrap();
    let (_, alert_dispatch) = use_store::<AlertsStore>();

    use_effect(move || {
        if !auth_store.loading && !auth_store.is_author() {
            alert_dispatch.reduce_mut(|alert_state| {
                *alert_state = AlertsStoreBuilder::new_error("Only Authors can create courses")
            });
            navigator.push(&Routes::Home);
        }

        || {}
    });

    html! {
        <BBContainer margin={BBContainerMargin::Normal}>
            <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>{"Create Course"}</BBTitle>
            <BBTextArea
                id="long-description"
                label="Long Description"
                rows=5
            />
            <BBInput
                id="price"
                label="Price (In Dollars)"
                name="price"
                input_type={BBInputType::Number}
            />
            <BBInput
                id="short-description"
                label="Short Description"
                name="short_description"
            />
        </BBContainer>
    }
}
