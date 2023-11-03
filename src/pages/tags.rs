use crate::{
    router::Routes,
    stores::main_store::{self, MainStore},
};
use std::ops::Deref;
use web_sys::FormData;
use ycl::{
    components::data_table::BBDataTable,
    elements::{
        button::{BBButton, BBButtonStyle, BBButtonType},
        form::BBForm,
        input::BBInput,
        table::BBTableRow,
        title::{BBTitle, BBTitleLevel},
    },
    foundations::{
        align_text::AlignText,
        container::{BBContainer, BBContainerMargin},
        states::BBLoadingState,
    },
};
use yew::{function_component, html, use_effect, use_state, AttrValue, Callback, Html};
use yew_router::prelude::use_navigator;
use yewdux::prelude::use_store;

#[function_component(Tags)]
pub fn component() -> Html {
    let navigator = use_navigator().unwrap();
    let (store, dispatch) = use_store::<MainStore>();
    let tag_value = use_state(AttrValue::default);

    {
        let store = store.clone();
        let dispatch = dispatch.clone();

        use_effect(move || {
            let done = || {};

            match store.auth_loaded {
                BBLoadingState::Initialized => {}
                BBLoadingState::Loading => {}
                BBLoadingState::Loaded => {
                    if !store.user.is_author() {
                        main_store::error_alert(dispatch, "Only Authors can manage tags");
                        navigator.push(&Routes::Home);
                    }
                }
            }

            done
        });
    }

    let all_tags_titles = vec!["Tag Name".into()];

    let all_tags_rows = store
        .tags
        .values()
        .map(|tag| BBTableRow {
            id: tag.id.to_string().into(),
            values: vec![tag.name.clone()],
            slot: None,
        })
        .collect::<Vec<BBTableRow>>();

    let new_tag_onsubmit = {
        let tag_value = tag_value.clone();

        Callback::from(move |event: FormData| {
            let dispatch = dispatch.clone();
            let Some(tag_name) = event.get("tag_name").as_string() else {
                main_store::set_alert(dispatch, "Missing tag name");

                return;
            };
            let tag_value = tag_value.clone();

            wasm_bindgen_futures::spawn_local(async move {
                main_store::insert_tag(dispatch, tag_name.into()).await;
                tag_value.set(AttrValue::default());
            });
        })
    };

    let tag_oninput = {
        let value = tag_value.clone();

        Callback::from(move |tag: AttrValue| {
            value.set(tag);
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
                    value={tag_value.deref().clone()}
                    oninput={tag_oninput}
                />
                <BBButton button_style={BBButtonStyle::PrimaryLight} button_type={BBButtonType::Submit}>{"Create Tag"}</BBButton>
            </BBForm>

            // table no longer takes values, but a Vec<BBTableRow>
            // probably want to switch to a data table anyways
            <BBDataTable
                title="Tags"
                title_level={BBTitleLevel::Two}
                id="all-tags"
                titles={all_tags_titles}
                rows={all_tags_rows}
            />
        </BBContainer>
    }
}
