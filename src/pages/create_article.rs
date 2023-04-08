use std::ops::Deref;

use ycl::{
    elements::{
        button::{BBButton, BBButtonStyle},
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

#[function_component(CreateArticle)]
pub fn component() -> Html {
    let title = use_state(|| AttrValue::from(""));
    let title_onchange = {
        let title = title.clone();
        Callback::from(move |new_title: AttrValue| {
            title.set(new_title);
        })
    };

    html! {
        <BBContainer margin={BBContainerMargin::Normal}>
            <BBTitle align={AlignText::Center} level={BBTitleLevel::One}>{"Articles"}</BBTitle>
            <BBForm>
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
                    name="body"
                />
                <BBButton button_style={BBButtonStyle::PrimaryLight}>{"Create Article"}</BBButton>
            </BBForm>
        </BBContainer>
    }
}
