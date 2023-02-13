use ycl::elements::title::{BBTitle, BBTitleLevel};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: i64,
}

#[function_component(CourseProfile)]
pub fn component(props: &Props) -> Html {
    html! {
        <BBTitle level={BBTitleLevel::One}>{"Course Page"}</BBTitle>
    }
}
