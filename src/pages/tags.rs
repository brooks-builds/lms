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
use yew_router::prelude::use_navigator;
use yewdux::prelude::use_store;

use crate::{
    router::Routes,
    stores::main_store::{self, MainStore},
};

#[function_component(Tags)]
pub fn component() -> Html {
    let navigator = use_navigator().unwrap();
    let (store, dispatch) = use_store::<MainStore>();

    if store.logged_in() && !store.user.is_author() {
        main_store::error_alert(dispatch.clone(), "Only Authors can manage tags");
        navigator.push(&Routes::Home);
    }

    let tag_titles = vec!["Tag Name".into()];

    let tag_values = store
        .tags
        .values()
        .map(|tag| {
            let mut row = HashMap::new();
            row.insert("Tag Name".into(), tag.name.clone());
            row
        })
        .collect::<Vec<HashMap<AttrValue, AttrValue>>>();

    let new_tag_state = use_state(|| AttrValue::from(""));

    let new_tag_onsubmit = Callback::from(move |event: FormData| {
        let dispatch = dispatch.clone();
        let Some(tag_name) = event.get("tag_name").as_string() else {
                main_store::set_alert(dispatch, "Missing tag name".into());

                return;
            };

        wasm_bindgen_futures::spawn_local(async move {
            main_store::insert_tag(dispatch, tag_name.into()).await;
        });
    });

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
