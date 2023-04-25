#![allow(non_camel_case_types)]
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub course_id: i64,
    pub article_id: i64,
}

#[function_component(CourseAccessArticle)]
pub fn component(_props: &Props) -> Html {
    html! {}
}
