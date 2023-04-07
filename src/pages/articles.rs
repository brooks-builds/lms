use ycl::{
    elements::{
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

#[function_component(Articles)]
pub fn component() -> Html {
    html! {
        <BBContainer margin={BBContainerMargin::Normal}>
            <BBTitle align={AlignText::Center} level={BBTitleLevel::One}>{"Articles"}</BBTitle>
            <BBForm>
                <BBInput
                    id="title"
                    label="Title"
                    name="title"
                />
                <BBTexArea
                    id="body"
                    label="Article Body"
                    name="body"
                />
            </BBForm>
        </BBContainer>
    }
}
