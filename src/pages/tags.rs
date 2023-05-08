use std::{collections::HashMap, ops::Deref};

use web_sys::FormData;
use ycl::{
    elements::{
        button::{BBButton, BBButtonStyle, BBButtonType},
        form::BBForm,
        input::BBInput,
        table::BBTable,
        title::{BBTitle, BBTitleLevel},
    },
    foundations::{
        align_text::AlignText,
        container::{BBContainer, BBContainerMargin},
    },
};
use yew::{function_component, html, use_state, AttrValue, Callback, Html};
use yew_hooks::use_effect_once;
use yew_router::prelude::use_navigator;
use yewdux::prelude::use_store;

use crate::{
    router::Routes,
    stores::{
        alerts::{AlertsStore, AlertsStoreBuilder},
        auth_store::AuthStore,
        courses_store::CourseStore,
        main_store::MainStore,
    },
};

#[function_component(Tags)]
pub fn component() -> Html {
    let (_, alert_dispatch) = use_store::<AlertsStore>();
    let (auth_store, _) = use_store::<AuthStore>();
    let navigator = use_navigator().unwrap();
    let (store, dispatch) = use_store::<MainStore>();

    if !auth_store.loading && !auth_store.is_author() {
        alert_dispatch.reduce_mut(|alert_state| {
            *alert_state = AlertsStoreBuilder::new_error("Only Authors can manage tags")
        });
        navigator.push(&Routes::Home);
    }

    let (courses_store, courses_dispatch) = use_store::<CourseStore>();

    {
        let alert_dispatch = alert_dispatch.clone();
        let courses_dispatch = courses_dispatch.clone();
        use_effect_once(move || {
            let courses_dispatch = courses_dispatch.clone();

            wasm_bindgen_futures::spawn_local(async move {});

            || ()
        });
    }

    let tag_titles = vec!["Tag Name".into()];

    let tag_values = store
        .tags
        .iter()
        .map(|(id, tag)| {
            let mut row = HashMap::new();
            row.insert("Tag Name".into(), tag.name.clone().into());
            row
        })
        .collect::<Vec<HashMap<AttrValue, AttrValue>>>();

    let new_tag_state = use_state(|| AttrValue::from(""));

    let new_tag_onsubmit = {
        let new_tag_state = new_tag_state.clone();
        let alert_dispatch = alert_dispatch;
        let courses_dispatch = courses_dispatch;
        let auth_store = auth_store;

        Callback::from(move |event: FormData| {
            let tag_name = if let Some(name) = event.get("tag_name").as_string() {
                if name.is_empty() {
                    alert_dispatch.clone().reduce_mut(|alert_store| {
                        *alert_store = AlertsStoreBuilder::new()
                            .icon(ycl::elements::icon::BBIconType::Warning)
                            .message("Cannot create a tag without a name")
                            .alert_type(ycl::modules::banner::BBBannerType::Error)
                            .build()
                            .unwrap();
                    });
                    return;
                }
                name
            } else {
                alert_dispatch.clone().reduce_mut(|alert_store| {
                    *alert_store = AlertsStoreBuilder::new()
                        .icon(ycl::elements::icon::BBIconType::Warning)
                        .message("Error creating new tag")
                        .alert_type(ycl::modules::banner::BBBannerType::Error)
                        .build()
                        .unwrap();
                });
                return;
            };
            let new_tag_state = new_tag_state.clone();
            let alert_dispatch = alert_dispatch.clone();
            let courses_dispatch = courses_dispatch.clone();
            let token = if let Some(token) = &auth_store.access_token {
                token.clone()
            } else {
                alert_dispatch.reduce_mut(|alert_state| {
                    *alert_state =
                        AlertsStoreBuilder::new_error("Must be logged in to create a tag");
                });
                return;
            };

            wasm_bindgen_futures::spawn_local(async move {
                new_tag_state.set(AttrValue::from(""));
            });
        })
    };

    let new_tag_onchange = {
        let new_tag_state = new_tag_state.clone();
        Callback::from(move |event: AttrValue| {
            new_tag_state.set(event);
        })
    };

    html! {
        <BBContainer margin={BBContainerMargin::Normal}>
            <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>{"Course Tags"}</BBTitle>
            <BBTitle level={BBTitleLevel::Two}>{"Create Tag"}</BBTitle>
            <BBForm onsubmit={new_tag_onsubmit}>
                <BBInput
                    id="tag-name"
                    label="Tag Name"
                    name="tag_name"
                    value={new_tag_state.deref()}
                    onchange={new_tag_onchange}
                />
                <BBButton button_style={BBButtonStyle::PrimaryLight} button_type={BBButtonType::Submit}>{"Create Tag"}</BBButton>
            </BBForm>

            <BBTable titles={tag_titles} values={tag_values} />
        </BBContainer>
    }
}
