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
};
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::prelude::use_store;

use crate::{
    router::Routes,
    stores::{
        alerts::{AlertsStore, AlertsStoreBuilder},
        auth_store::AuthStore,
        main_store::{self, MainStore},
    },
};

#[function_component(CreateArticle)]
pub fn component() -> Html {
    let (store, dispatch) = use_store::<MainStore>();
    let title = use_state(|| AttrValue::from(""));
    let title_onchange = {
        let title = title.clone();
        Callback::from(move |new_title: AttrValue| {
            title.set(new_title);
        })
    };

    let (auth, _) = use_store::<AuthStore>();
    let (_, alert_dispatch) = use_store::<AlertsStore>();

    let navigator = use_navigator().unwrap();

    {
        let auth = auth.clone();
        let dispatch = dispatch.clone();

        use_effect(move || {
            if !auth.loading && !auth.is_author() {
                main_store::error_alert(dispatch, "Only Authors can create articles");
                navigator.push(&Routes::Home);
            }

            || {}
        })
    }

    let onsubmit = {
        let title_state = title.clone();

        Callback::from(move |form: FormData| {
            let Some(title )= form.get("title").as_string() else {
                main_store::error_alert(dispatch.clone(), "missing title");
                return;
            };
            if title.is_empty() {
                main_store::error_alert(dispatch.clone(), "Title cannot be empty");
                return;
            }

            let Some(content) = form.get("content").as_string() else {
                main_store::error_alert(dispatch.clone(), "Missing Content");
                return
            };
            if content.is_empty() {
                main_store::error_alert(dispatch.clone(), "Content cannot be empty");
                return;
            }

            {
                let dispatch = dispatch.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    main_store::insert_article(dispatch, title.into(), content.into()).await
                });
            }
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
