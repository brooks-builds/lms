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
use yewdux::prelude::use_store;

use crate::{
    api,
    logging::log_error,
    stores::{
        alerts::{AlertsStore, AlertsStoreBuilder},
        courses_store::CourseStore,
    },
};

#[function_component(Tags)]
pub fn component() -> Html {
    let (courses_store, courses_dispatch) = use_store::<CourseStore>();
    let (_, alert_dispatch) = use_store::<AlertsStore>();

    {
        let alert_dispatch = alert_dispatch.clone();
        let courses_dispatch = courses_dispatch.clone();
        use_effect_once(move || {
            let courses_dispatch = courses_dispatch.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match api::courses::get_tags().await {
                    Ok(store_tags) => courses_dispatch.reduce_mut(|course_store| {
                        course_store.tags = store_tags;
                    }),
                    Err(error) => {
                        log_error("Error getting tags", &error);
                        alert_dispatch.reduce_mut(|alert_store| {
                            *alert_store = AlertsStoreBuilder::new()
                                .icon(ycl::elements::icon::BBIconType::Warning)
                                .message("Error getting tags")
                                .alert_type(ycl::modules::banner::BBBannerType::Error)
                                .build()
                                .unwrap();
                        })
                    }
                }
            });

            || ()
        });
    }

    let tag_titles = vec!["Tag Name".into()];

    let tag_values = courses_store
        .tags
        .iter()
        .map(|store_tag| {
            let mut row = HashMap::new();
            row.insert("Tag Name".into(), store_tag.name.clone().into());
            row
        })
        .collect::<Vec<HashMap<AttrValue, AttrValue>>>();

    let new_tag_state = use_state(|| AttrValue::from(""));

    let new_tag_onsubmit = {
        let new_tag_state = new_tag_state.clone();
        let alert_dispatch = alert_dispatch.clone();
        let courses_dispatch = courses_dispatch.clone();

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

            wasm_bindgen_futures::spawn_local(async move {
                match api::tags::insert_tag(tag_name).await {
                    Ok(tag) => courses_dispatch.reduce_mut(move |courses_store| {
                        let tag = if let Some(tag) = tag.insert_lms_tags_one {
                            tag
                        } else {
                            alert_dispatch.clone().reduce_mut(move |alert_store| {
                                *alert_store = AlertsStoreBuilder::new()
                                    .icon(ycl::elements::icon::BBIconType::Warning)
                                    .message("Error creating new tag")
                                    .alert_type(ycl::modules::banner::BBBannerType::Error)
                                    .build()
                                    .unwrap();
                            });
                            return;
                        };

                        courses_store
                            .tags
                            .push(crate::stores::courses_store::StoreTag {
                                id: tag.id,
                                name: tag.name,
                            })
                    }),
                    Err(error) => {
                        log_error("error creating new tag", &error);
                        alert_dispatch.reduce_mut(move |alert_store| {
                            *alert_store = AlertsStoreBuilder::new()
                                .icon(ycl::elements::icon::BBIconType::Warning)
                                .message("Error creating new tag")
                                .alert_type(ycl::modules::banner::BBBannerType::Error)
                                .build()
                                .unwrap();
                        });
                    }
                }
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
