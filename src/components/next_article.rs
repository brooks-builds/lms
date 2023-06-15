#![allow(non_camel_case_types)]
use ycl::elements::{
    button::{BBButton, BBButtonStyle},
    internal_link::BBInternalLink,
};
use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::{
    router::Routes,
    stores::main_store::MainStore,
    types::{Article, Course},
};

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub article_id: i64,
    pub course_id: i64,
    #[prop_or_default()]
    pub onclick: Callback<i64>,
}

#[function_component(NextArticle)]
pub fn component(props: &Props) -> Html {
    let (store, _dispatch) = use_store::<MainStore>();
    let Some(course) = store.courses.get(&props.course_id) else { return html! {} };
    let props_onclick = props.onclick.clone();
    let article_id = props.article_id;
    let onclick = Callback::from(move |_| {
        props_onclick.emit(article_id);
    });
    let Some(next_article) = next_article(course, props.article_id, store.own_course(props.course_id)) else { return html! {
        <BBButton onclick={onclick.clone()} button_style={BBButtonStyle::PrimaryLight}>{"Complete Article"}</BBButton>
    } };
    let title = format!("Complete and goto next article: {}", &next_article.title);

    html! {
        <BBInternalLink<Routes> to={Routes::CourseAccessArticle { course_id: props.course_id, article_id: next_article.id }} button={true} {onclick}>{title}</BBInternalLink<Routes>>
    }
}

fn next_article(course: &Course, article_id: i64, own_course: bool) -> Option<&Article> {
    let (current_index, _current_article) = course
        .articles
        .iter()
        .enumerate()
        .find(move |(_, article)| article.id == article_id)?;

    if own_course {
        course.articles.get(current_index + 1)
    } else {
        course
            .articles
            .iter()
            .find(move |article| article.id > article_id && article.preview.unwrap_or_default())
    }
}
