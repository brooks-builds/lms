use ycl::{foundations::container::BBContainer, modules::hero::BBHero};
use yew::prelude::*;

#[function_component(Home)]
pub fn component() -> Html {
    html! {
        <BBContainer>
            <BBHero text="Welcome to Brooks Builds Learning. Here you can find high-quality education-first courses on Rust"
                title="Brooks Builds Learning"
            />
        </BBContainer>
    }
}
