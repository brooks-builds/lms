use ycl::{
    elements::title::{BBTitle, BBTitleLevel},
    foundations::{align_text::AlignText, container::BBContainer},
    modules::card_list::{BBCardData, BBCardDataBuilder, BBCardList},
};
use yew::{function_component, html, Html};
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

    let tag_cards = courses_store
        .tags
        .iter()
        .map(|tag| {
            BBCardDataBuilder::<Routes>::new()
                .tag(tag.name.as_str().into())
                .title(&tag.name)
                .build()
        })
        .collect::<Vec<_>>();

    html! {
        <BBContainer>
            <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>{"Course Tags"}</BBTitle>
            <BBCardList<Routes>
                card_data={tag_cards}
                card_title_level={BBTitleLevel::Two}
                action="Create Tag"
                title="Tags"
                more={true}
            />
        </BBContainer>
    }
}
