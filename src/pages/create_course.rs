use std::ops::Deref;

use web_sys::FormData;
use ycl::{
    elements::{
        button::{BBButton, BBButtonStyle, BBButtonType},
        form::BBForm,
        input::{BBInput, BBInputType},
        text_area::BBTextArea,
        title::{BBTitle, BBTitleLevel},
    },
    foundations::{
        align_text::AlignText,
        container::{BBContainer, BBContainerMargin},
    },
    modules::select::{BBOption, BBSelect},
};
use yew::{function_component, html, use_effect, use_state, AttrValue, Callback, Html};
use yew_hooks::use_effect_once;
use yew_router::prelude::use_navigator;
use yewdux::prelude::use_store;

use crate::{
    api,
    logging::{log_data, log_error},
    router::Routes,
    stores::{
        alerts::{AlertsStore, AlertsStoreBuilder},
        auth_store::AuthStore,
        courses_store::{CourseStore, StoreTag},
    },
};

#[function_component(CreateCourse)]
pub fn component() -> Html {
    let (auth_store, _) = use_store::<AuthStore>();
    let navigator = use_navigator().unwrap();
    let (_, alert_dispatch) = use_store::<AlertsStore>();
    let (courses_store, courses_dispatch) = use_store::<CourseStore>();
    let title = use_state(|| AttrValue::from(""));
    let short_description = use_state(|| AttrValue::from(""));

    {
        let alert_dispatch = alert_dispatch.clone();
        use_effect(move || {
            if !auth_store.loading && !auth_store.is_author() {
                alert_dispatch.reduce_mut(|alert_state| {
                    *alert_state = AlertsStoreBuilder::new_error("Only Authors can create courses")
                });
                navigator.push(&Routes::Home);
            }

            || {}
        });
    }

    use_effect_once(move || {
        let alert_dispatch = alert_dispatch.clone();
        let courses_dispatch = courses_dispatch.clone();

        wasm_bindgen_futures::spawn_local(async move {
            match api::tags::get_tags().await {
                Ok(tags) => courses_dispatch.reduce_mut(|course_state| {
                    course_state.tags = tags
                        .lms_tags
                        .into_iter()
                        .map(|tag| StoreTag {
                            id: tag.id,
                            name: tag.name,
                        })
                        .collect()
                }),
                Err(error) => {
                    log_error("Error getting tags", &error);
                    alert_dispatch.reduce_mut(|state| {
                        *state = AlertsStoreBuilder::new_error("Error loading all tags")
                    });
                }
            }
        });

        || {}
    });

    let onsubmit = Callback::from(|event: FormData| {
        let title = match event.get("title").as_string() {
            Some(value) => value,
            None => 
        }
        let tag = event.get("tag");
        let long_description = event.get("long_description");
        let short_description = event.get("short_description");

        log_data("title", title);
        log_data("tag", tag);
        log_data("long_description", long_description);
        log_data("short_description", short_description);
    });

    let title_onchange = {
        let title = title.clone();

        Callback::from(move |event: AttrValue| {
            title.set(event);
        })
    };

    let short_description_onchange = {
        let short_description = short_description.clone();

        Callback::from(move |event: AttrValue| {
            short_description.set(event);
        })
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
                    onchange={title_onchange}
                />
                <BBSelect
                    id="tag"
                    label="Tag"
                    options={courses_store.tags.iter().map(|tag| BBOption {value: tag.id.to_string().into(), label: tag.name.clone().into()}).collect::<Vec<BBOption>>()}
                    name="tag"
                 />
                <BBTextArea
                    id="long-description"
                    label="Long Description"
                    rows=5
                    name="long_description"
                />
                <BBInput
                    id="short-description"
                    label="Short Description"
                    name="short_description"
                    value={short_description.deref().clone()}
                    onchange={short_description_onchange}
                />
                <BBButton button_type={BBButtonType::Submit} button_style={BBButtonStyle::PrimaryLight}>{"Create Course"}</BBButton>
            </BBForm>
        </BBContainer>
    }
}
