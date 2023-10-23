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
        states::BBLoadingState,
    },
};
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::prelude::use_store;

use crate::{
    router::Routes,
    stores::main_store::{self, MainStore},
};

#[function_component(CreateArticle)]
pub fn component() -> Html {
    let (store, dispatch) = use_store::<MainStore>();
    let title = use_state(|| AttrValue::from(""));
    let navigator = use_navigator().unwrap();

    {
        let dispatch = dispatch.clone();

        use_effect(move || {
            if store.auth_loaded == BBLoadingState::Loaded && !store.user.is_author() {
                main_store::error_alert(dispatch, "Only Authors can create articles");
                navigator.push(&Routes::Home);
            }

            || {}
        })
    }

    let content_value = use_state(|| AttrValue::from(""));
    let content_oninput = {
        let value = content_value.clone();

        Callback::from(move |new_value| value.set(new_value))
    };

    let onsubmit = {
        let title_store = title.clone();
        let content_value = content_value.clone();

        Callback::from(move |form: FormData| {
            let Some(title) = form.get("title").as_string() else {
                main_store::error_alert(dispatch.clone(), "missing title");
                return;
            };
            if title.is_empty() {
                main_store::error_alert(dispatch.clone(), "Title cannot be empty");
                return;
            }

            let Some(content) = form.get("content").as_string() else {
                main_store::error_alert(dispatch.clone(), "Missing Content");
                return;
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

            title_store.set(AttrValue::from(""));
            content_value.set(AttrValue::from(""));
        })
    };

    let title_oninput = {
        let title = title.clone();

        Callback::from(move |new_title: AttrValue| title.set(new_title))
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
                    oninput={title_oninput}
                />
                <BBTextArea
                    id="body"
                    label="Article Body"
                    name="content"
                    value={content_value.deref().clone()}
                    oninput={content_oninput}
                />
                <BBButton button_style={BBButtonStyle::PrimaryLight} button_type={BBButtonType::Submit}>{"Create Article"}</BBButton>
            </BBForm>
        </BBContainer>
    }
}
