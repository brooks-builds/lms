use ycl::modules::nav::course_nav::{BBCourseNav, BBCourseNavArticle, BBCourseNavArticleBuilder};
use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::{
    logging::log_data,
    router::Routes,
    stores::{
        alerts::{AlertsStore, AlertsStoreBuilder},
        articles::{Article, ArticlesStore},
        courses_store::{CourseStore, StoreCourse},
    },
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub course_id: i64,
}

#[function_component(CourseNav)]
pub fn component(props: &Props) -> Html {
    let (course_store, _) = use_store::<CourseStore>();
    let (_, alert_dispatch) = use_store::<AlertsStore>();
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
                log_data("loading course for nav", &store_course.name);
                course.set(Some(store_course.clone()));
            }

            result
        });
    }

    if let Some(course) = &*course {
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

        html! {
            <BBCourseNav<Routes> {articles} />
        }
    } else {
        html! {}
    }
}

fn article_title(article: &Article) -> String {
    let preview = if article.preview { "(preview)" } else { "" };

    format!("{} {preview}", article.title)
}
