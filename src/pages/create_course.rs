use std::{collections::HashMap, ops::Deref};

use web_sys::FormData;
use ycl::{
    elements::{
        button::{BBButton, BBButtonStyle, BBButtonType},
        checkbox::BBCheckbox,
        form::BBForm,
        input::BBInput,
        text_area::BBTextArea,
        title::{BBTitle, BBTitleLevel},
    },
    foundations::{
        align_text::AlignText,
        container::{BBContainer, BBContainerMargin},
        states::{BBLoadingState, BBValidationState},
    },
    modules::select::{BBOption, BBSelect},
};
use yew::{function_component, html, use_effect, use_state, AttrValue, Callback, Html};
use yew_router::prelude::use_navigator;
use yewdux::prelude::use_store;

use crate::{
    router::Routes,
    stores::main_store::{self, MainStore},
    types::Tag,
};

#[function_component(CreateCourse)]
pub fn component() -> Html {
    let navigator = use_navigator().unwrap();
    let title = use_state(|| AttrValue::from(""));
    let short_description = use_state(|| AttrValue::from(""));
    let (store, dispatch) = use_store::<MainStore>();
    let title_isvalid = use_state(|| BBValidationState::Initialized);

    {
        let store = store.clone();
        let dispatch = dispatch.clone();

        use_effect(move || {
            if store.courses_loaded == BBLoadingState::Loaded && !store.user.is_author() {
                main_store::error_alert(dispatch, "Only Authors can create a course");
                navigator.push(&Routes::Home);
            }

            || {}
        });
    }

    let long_description_value = use_state(|| AttrValue::from(""));
    let long_description_oninput = {
        let value = long_description_value.clone();

        Callback::from(move |new_value| value.set(new_value))
    };

    let onsubmit = {
        let long_description_value = long_description_value.clone();

        Callback::from(move |event: FormData| {
            let Some(tag) = event.get("tag").as_string() else {
                main_store::set_alert(dispatch.clone(), "missing tag id");
                return;
            };
            let Ok(tag_id) = tag.parse::<i64>() else {
                main_store::set_alert(dispatch.clone(), "tag id is not a number");
                return;
            };
            let Some(title) = event.get("title").as_string() else {
                return;
            };
            let Some(long_description) = event.get("long_description").as_string() else {
                return;
            };
            let Some(short_description) = event.get("short_description").as_string() else {
                return;
            };
            let live_course = event.get("live_course").as_string().is_some();

            let dispatch = dispatch.clone();

            wasm_bindgen_futures::spawn_local(async move {
                main_store::insert_course(
                    dispatch,
                    long_description.into(),
                    title.into(),
                    tag_id,
                    short_description.into(),
                    live_course,
                )
                .await;
            });

            long_description_value.set(AttrValue::from(""));
        })
    };

    let title_oninput = {
        let title = title.clone();

        Callback::from(move |new_title: AttrValue| title.set(new_title))
    };

    let short_description_oninput = {
        let short_description = short_description.clone();

        Callback::from(move |new_short_description: AttrValue| {
            short_description.set(new_short_description)
        })
    };

    let title_onisvalid = {
        let title_isvalid = title_isvalid.clone();

        Callback::from(move |valid: BBValidationState| title_isvalid.set(valid))
    };

    html! {
        <BBContainer margin={BBContainerMargin::Normal}>
            <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>{"Create Course"}</BBTitle>
            <BBForm {onsubmit}>
                <BBInput
                    id="title"
                    label="Title"
                    name="title"
                    value={title.deref().clone()}
                    oninput={title_oninput}
                    onisvalid={title_onisvalid}
                    is_valid={title_isvalid.deref().clone()}
                />
                <BBSelect
                    id="tag"
                    label="Tag"
                    options={create_tag_options(&store.tags)}
                    name="tag"
                 />
                <BBTextArea
                    id="long-description"
                    label="Long Description"
                    rows=5
                    name="long_description"
                    value={long_description_value.deref().clone()}
                    oninput={long_description_oninput}
                />
                <BBInput
                    id="short-description"
                    label="Short Description"
                    name="short_description"
                    value={short_description.deref().clone()}
                    oninput={short_description_oninput}
                />
                <BBContainer>
                    <BBCheckbox
                        id="live_course"
                        value="live_course"
                        label="live course"
                        name="live_course"
                    />
                </BBContainer>
                <BBButton button_type={BBButtonType::Submit} button_style={BBButtonStyle::PrimaryLight} disabled={title_isvalid.not_valid()}>{"Create Course"}</BBButton>
            </BBForm>
        </BBContainer>
    }
}

fn create_tag_options(tags: &HashMap<i64, Tag>) -> Vec<BBOption> {
    let mut tag_options = tags
        .iter()
        .map(|(id, tag)| BBOption {
            value: id.to_string().into(),
            label: tag.name.clone(),
        })
        .collect::<Vec<BBOption>>();
    tag_options.sort_by(|a, b| a.partial_cmp(b).unwrap());
    tag_options
}
