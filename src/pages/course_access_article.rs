#![allow(non_camel_case_types)]
use ycl::{
    elements::title::{BBTitle, BBTitleLevel},
    foundations::{
        align_text::AlignText,
        column::{BBCol, BBColWidth},
        container::BBContainer,
        row::BBRow,
        states::BBLoadingState,
    },
    modules::course_content::BBCourseContent,
};
use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::{
    api,
    components::course_nav::CourseNav,
    logging::log_error,
    stores::{
        alerts::{AlertsStore, AlertsStoreBuilder},
        articles::{Article, ArticlesStore},
        courses_store::{CourseStore, StoreCourse},
    },
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub course_id: i64,
    pub article_id: i64,
}

#[function_component(CourseAccessArticle)]
pub fn component(props: &Props) -> Html {
    let (store_course, store_dispatch) = use_store::<CourseStore>();
    let (_, alert_dispatch) = use_store::<AlertsStore>();
    let course_loaded_flag = use_state(|| BBLoadingState::Initialized);
    let course: UseStateHandle<Option<StoreCourse>> = use_state(|| None);
    let articles_loaded_flag = use_state(|| BBLoadingState::Initialized);
    let (_, articles_dispatch) = use_store::<ArticlesStore>();
    let article_state: UseStateHandle<Option<Article>> = use_state(|| None);

    {
        let course_id = props.course_id;
        let course = course.clone();
        let article_id = props.article_id;
        let article_state = article_state.clone();

        use_effect(move || {
            let result = || {};

            if *course_loaded_flag == BBLoadingState::Loaded
                && *articles_loaded_flag == BBLoadingState::Loaded
            {
                return result;
            }

            if *articles_loaded_flag == BBLoadingState::Initialized
                && *course_loaded_flag == BBLoadingState::Loaded
            {
                articles_loaded_flag.set(BBLoadingState::Loading);
                let article_ids = vec![];
                let alert_dispatch = alert_dispatch.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match api::articles::get_article_titles_by_ids(article_ids).await {
                        Ok(articles) => {
                            articles_dispatch.reduce_mut(|articles_state| {
                                for article in articles.clone() {
                                    articles_state.articles.insert(article.id, article.clone());
                                }
                            });
                            article_state.clone().set(
                                articles
                                    .iter()
                                    .find(|article| article.id == article_id)
                                    .cloned(),
                            );
                            articles_loaded_flag.set(BBLoadingState::Loaded);
                        }
                        Err(error) => {
                            log_error("Error getting articles for access course article", &error);
                            alert_dispatch.reduce(|_| {
                                AlertsStoreBuilder::new_error("Error loading article nav").into()
                            });
                        }
                    }
                });
            }

            if *course_loaded_flag == BBLoadingState::Initialized {
                course_loaded_flag.set(BBLoadingState::Loading);

                if let Some(store_course) = store_course.courses.get(&course_id) {
                    course.set(Some(store_course.clone()));
                    course_loaded_flag.set(BBLoadingState::Loaded);
                    return result;
                }

                let store_dispatch = store_dispatch.clone();
                let alert_dispatch = alert_dispatch.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    match api::courses::get_by_id(course_id).await {
                        Ok(db_course) => {
                            store_dispatch.reduce_mut(|store_state| {
                                store_state.courses.insert(db_course.id, db_course.clone());
                            });
                            course.set(Some(db_course));
                            course_loaded_flag.set(BBLoadingState::Loaded);
                        }
                        Err(error) => {
                            log_error(
                                "Error getting course by id for course access article",
                                &error,
                            );
                            alert_dispatch.reduce(|_| {
                                AlertsStoreBuilder::new_error("Error getting course").into()
                            });
                        }
                    }
                });
            }

            result
        });
    }

    if let Some(course) = &*course {
        if let Some(article) = &*article_state {
            html! {
                <BBContainer>
                    <BBTitle align={AlignText::Center} level={BBTitleLevel::One}>
                        {&course.name}
                    </BBTitle>
                    <BBRow>
                        <BBCol width={BBColWidth::Three}>
                            <CourseNav course_id={props.course_id} />
                        </BBCol>
                        <BBCol>
                            <BBCourseContent
                                have_access={false}
                                course={article.content.clone().unwrap_or_default()}
                            />
                        </BBCol>
                    </BBRow>
                </BBContainer>
            }
        } else {
            loading()
        }
    } else {
        loading()
    }
}

fn loading() -> Html {
    html! {
        <BBContainer>
            <BBTitle align={AlignText::Center} level={BBTitleLevel::One}>
                {"Loading"}
            </BBTitle>
        </BBContainer>

    }
}
