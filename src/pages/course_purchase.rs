#![allow(non_camel_case_types)]
use yew::prelude::*;

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub course_id: i64,
}

#[function_component(CoursePurchase)]
pub fn component(_props: &Props) -> Html {
    html! {}
}
