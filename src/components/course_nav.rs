#![allow(non_camel_case_types)]
use ycl::modules::nav::course_nav::{BBCourseNav, BBCourseNavArticle, BBCourseNavArticleBuilder};
use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::{router::Routes, stores::main_store::MainStore};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub course_id: i64,
    pub preview_articles: Vec<i64>,
}

#[function_component(CourseNav)]
pub fn component(props: &Props) -> Html {
    let (store, _dispatch) = use_store::<MainStore>();

    if let Some(course) = store.courses.get(&props.course_id) {
        let course_id = props.course_id;
        let articles = {
            let store = store.clone();
            course
                .articles
                .iter()
                .map(move |article| {
                    let is_preview = props.preview_articles.contains(&article.id);
                    let is_owned = store.own_course(course_id);
                    let mut article_builder = BBCourseNavArticleBuilder::new()
                        .title(article.title.clone())
                        .id(article.id.to_string())
                        .preview(if is_owned { false } else { is_preview });

                    article_builder = if let Some(db_user) = &store.db_user {
                        article_builder.completed(db_user.has_completed_article(article.id))
                    } else {
                        article_builder
                    };

                    article_builder = if is_preview || is_owned {
                        article_builder.to(Routes::CourseAccessArticle {
                            course_id: props.course_id,
                            article_id: article.id,
                        })
                    } else {
                        article_builder
                    };

                    article_builder.build().unwrap()
                })
                .collect::<Vec<BBCourseNavArticle<Routes>>>()
        };

        html! {
            <BBCourseNav<Routes> {articles} />
        }
    } else {
        html! {}
    }
}
