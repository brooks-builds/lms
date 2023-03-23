use std::collections::HashMap;

use ycl::{
    elements::{
        table::BBTable,
        title::{BBTitle, BBTitleLevel},
    },
    foundations::{
        align_text::AlignText,
        container::{BBContainer, BBContainerMargin},
    },
    modules::card_list::{BBCardData, BBCardDataBuilder, BBCardList},
};
use yew::{function_component, html, AttrValue, Html};
use yew_hooks::use_effect_once;
use yewdux::prelude::use_store;

use crate::{
    api,
    logging::log_error,
    router::Routes,
    stores::{
        alerts::{AlertsStore, AlertsStoreBuilder},
        courses_store::CourseStore,
    },
};

#[function_component(Tags)]
pub fn component() -> Html {
    let (courses_store, courses_dispatch) = use_store::<CourseStore>();
    let (_, alert_dispatch) = use_store::<AlertsStore>();

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

    html! {
        <BBContainer margin={BBContainerMargin::Normal}>
            <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>{"Course Tags"}</BBTitle>
            <BBTable titles={tag_titles} values={tag_values} />
        </BBContainer>
    }
}
