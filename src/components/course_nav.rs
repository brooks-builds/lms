#![allow(non_camel_case_types)]
use ycl::modules::nav::course_nav::{BBCourseNav, BBCourseNavArticle, BBCourseNavArticleBuilder};
use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::{
    logging::log_data,
    router::Routes,
    stores::{courses_store::CourseStore, main_store::MainStore},
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub course_id: i64,
}

#[function_component(CourseNav)]
pub fn component(props: &Props) -> Html {
    let (store, dispatch) = use_store::<MainStore>();

    if let Some(course) = store.courses.get(&props.course_id) {
        let articles = course
            .articles
            .iter()
            .map(|article| {
                BBCourseNavArticleBuilder::new()
                    .title(article.title.clone())
                    .to(Routes::CourseAccessArticle {
                        course_id: props.course_id,
                        article_id: article.id,
                    })
                    .build()
                    .unwrap()
            })
            .collect::<Vec<BBCourseNavArticle<Routes>>>();

        html! {
            <BBCourseNav<Routes> {articles} />
        }
    } else {
        html! {}
    }
}
