use std::ops::Deref;

use web_sys::FormData;
use ycl::{
    elements::{
        button::{BBButton, BBButtonStyle, BBButtonType},
        form::BBForm,
        input::BBInput,
        text_area::BBTextArea,
        title::{BBTitle, BBTitleLevel},
    },
    foundations::{
        align_text::AlignText,
        container::{BBContainer, BBContainerMargin},
    },
    modules::banner::BBBannerType,
};
use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::{
    api,
    logging::{log_data, log_error},
    stores::{
        alerts::{AlertsStore, AlertsStoreBuilder},
        auth_store::AuthStore,
    },
};

#[function_component(CreateArticle)]
pub fn component() -> Html {
    let title = use_state(|| AttrValue::from(""));
    let title_onchange = {
        let title = title.clone();
        Callback::from(move |new_title: AttrValue| {
            title.set(new_title);
        })
    };

    let (auth, _) = use_store::<AuthStore>();
    let (_, alert_dispatch) = use_store::<AlertsStore>();

    let onsubmit = {
        let title_state = title.clone();

        Callback::from(move |form: FormData| {
            let title = form.get("title").as_string().unwrap();
            let content = form.get("content").as_string().unwrap();
            let token = auth.access_token.clone().unwrap_or_default();
            let alert_dispatch = alert_dispatch.clone();
            let title_state = title_state.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match api::articles::create_article(title, content, token).await {
                    Ok(_article) => {
                        alert_dispatch.reduce_mut(|alert_state| {
                            *alert_state = AlertsStoreBuilder::new()
                                .message("Created Article")
                                .icon(ycl::elements::icon::BBIconType::Star)
                                .alert_type(BBBannerType::Success)
                                .build()
                                .unwrap()
                        });
                        title_state.set(AttrValue::default());
                    }
                    Err(error) => {
                        log_error("error creating article", &error);
                        alert_dispatch.reduce_mut(|alert_state| {
                            *alert_state = AlertsStoreBuilder::new_error(
                                "There was an error trying to create the article",
                            )
                        });
                    }
                }
            });
        })
    };

    html! {
        <BBContainer margin={BBContainerMargin::Normal}>
            <BBTitle align={AlignText::Center} level={BBTitleLevel::One}>{"Articles"}</BBTitle>
            <BBForm {onsubmit}>
                <BBInput
                    id="title"
                    label="Title"
                    name="title"
                    value={title.deref().clone()}
                    onchange={title_onchange}
                />
                <BBTextArea
                    id="body"
                    label="Article Body"
                    name="content"
                />
                <BBButton button_style={BBButtonStyle::PrimaryLight} button_type={BBButtonType::Submit}>{"Create Article"}</BBButton>
            </BBForm>
        </BBContainer>
    }
}
