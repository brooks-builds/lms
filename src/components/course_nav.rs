#![allow(non_camel_case_types)]
use ycl::modules::nav::course_nav::{BBCourseNav, BBCourseNavArticle, BBCourseNavArticleBuilder};
use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::{
    logging::log_data,
    router::Routes,
    stores::{
        articles::{Article, ArticlesStore},
        courses_store::CourseStore,
    },
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub course_id: i64,
}

#[function_component(CourseNav)]
pub fn component(props: &Props) -> Html {
    let (course_store, _) = use_store::<CourseStore>();
    let (articles_store, _) = use_store::<ArticlesStore>();
    let course = use_state(|| None);
    let course_id = props.course_id;

    {
        let course = course.clone();

        use_effect(move || {
            let result = || {};

            if course.is_some() {
                return result;
            }

            if let Some(store_course) = course_store.courses.get(&course_id) {
                log_data("found the course", store_course);
                course.set(Some(store_course.clone()));
            }

            result
        });
    }

    if let Some(course) = &*course {
        log_data("course", course);
        let articles = course
            .article_ids
            .iter()
            .filter_map(move |article_id| articles_store.articles.get(article_id).map(Clone::clone))
            .map(|article| {
                BBCourseNavArticleBuilder::new()
                    .title(article_title(&article))
                    .to(Routes::CourseAccessArticle {
                        course_id,
                        article_id: article.id,
                    })
                    .build()
                    .unwrap()
            })
            .collect::<Vec<BBCourseNavArticle<Routes>>>();

        log_data("we are in the if then", &articles);

        html! {
            <BBCourseNav<Routes> {articles} />
        }
    } else {
        log_data("we are in the if else", props.course_id);
        html! {}
    }
}

fn article_title(article: &Article) -> String {
    let preview = if article.preview { "(preview)" } else { "" };

    format!("{} {preview}", article.title)
}