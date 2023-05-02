use ycl::{
    elements::{
        text::BBText,
        title::{BBTitle, BBTitleLevel},
    },
    foundations::{
        align_text::AlignText,
        column::{BBCol, BBColWidth},
        container::{BBContainer, BBContainerMargin},
        row::BBRow,
        states::BBLoadingState,
    },
};
use yew::{function_component, html, use_effect, use_state, Html, Properties};
use yew_router::prelude::use_navigator;
use yewdux::prelude::use_store;

use crate::{
    api,
    components::course_nav::CourseNav,
    logging::log_error,
    router::Routes,
    stores::{
        alerts::{AlertsStore, AlertsStoreBuilder},
        articles::ArticlesStore,
        courses_store::CourseStore,
    },
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: i64,
}

#[function_component(CourseAccess)]
pub fn component(props: &Props) -> Html {
    let course_loading_state = use_state(|| BBLoadingState::Initialized);
    let (courses_store, courses_dispatch) = use_store::<CourseStore>();
    let course_id = props.id;
    let (_, alert_dispatch) = use_store::<AlertsStore>();
    let article_titles_loading_state = use_state(|| BBLoadingState::Initialized);
    let navigator = use_navigator().unwrap();
    let (_articles_store, articles_dispatch) = use_store::<ArticlesStore>();

    {
        let alert_dispatch = alert_dispatch;
        let courses_store = courses_store.clone();

        use_effect(move || {
            let result = || {};
            let alert_dispatch = alert_dispatch.clone();
            let courses_dispatch = courses_dispatch.clone();
            let course_loading_state = course_loading_state.clone();
            let courses_store = courses_store.clone();

            if *course_loading_state == BBLoadingState::Initialized {
                let course_loading_state = course_loading_state.clone();
                let alert_dispatch = alert_dispatch.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    course_loading_state.clone().set(BBLoadingState::Loading);
                    match api::courses::get_by_id(course_id).await {
                        Err(error) => {
                            log_error("error getting course", &error);
                            alert_dispatch.reduce_mut(|alert_state| {
                                *alert_state = AlertsStoreBuilder::new_error(
                                    "There was an error getting the course, please try again later",
                                )
                            });
                        }
                        Ok(api_course) => {
                            courses_dispatch.reduce_mut(|courses_state| {
                                courses_state.courses.insert(api_course.id, api_course);
                            });
                            course_loading_state.set(BBLoadingState::Loaded);
                        }
                    }
                });
            }

            if *article_titles_loading_state == BBLoadingState::Initialized
                && *course_loading_state == BBLoadingState::Loaded
            {
                let article_titles_loading_state = article_titles_loading_state.clone();
                let courses_store = courses_store;
                let Some(course) = courses_store.courses.get(&course_id) else {
                    alert_dispatch.reduce_mut(|alert_state| *alert_state = AlertsStoreBuilder::new_error("Could not find course"));
                    navigator.push(&Routes::Courses);
                    return result;
                };
                let article_ids = course.article_ids.clone();
                let articles_dispatch = articles_dispatch.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    article_titles_loading_state
                        .clone()
                        .set(BBLoadingState::Loading);
                    match api::articles::get_article_titles_by_ids(article_ids).await {
                        Ok(article_titles) => {
                            articles_dispatch.reduce_mut(move |articles_state| {
                                for article_title in article_titles {
                                    articles_state
                                        .articles
                                        .insert(article_title.id, article_title);
                                }
                            });
                            article_titles_loading_state.set(BBLoadingState::Loaded);
                        }
                        Err(error) => {
                            log_error("Error getting article titles", &error);
                            alert_dispatch.reduce_mut(|alert_state| {
                                *alert_state =
                                    AlertsStoreBuilder::new_error("Error getting articles")
                            });
                            navigator.push(&Routes::CourseDetails { id: course_id });
                        }
                    }
                });
            }

            result
        });
    }

    let Some(course) = courses_store.courses.get(&props.id) else {
        return html! {
        <BBContainer margin={BBContainerMargin::Normal}>
            <BBTitle align={AlignText::Center} level={BBTitleLevel::One}>{"Loading course"}</BBTitle>
        </BBContainer>
        };

    };

    html! {
        <BBContainer margin={BBContainerMargin::Normal}>
                    <BBTitle align={AlignText::Center} level={BBTitleLevel::One}>{&course.name}</BBTitle>
            <BBRow>
                <BBCol width={BBColWidth::Three}>
                    <CourseNav {course_id} />
                </BBCol>
                <BBCol>
                    <BBTitle
                        align={AlignText::Center}
                        level={BBTitleLevel::Two}
                    >
                        {"Select an Article"}
                    </BBTitle>
                    <BBText>
                        {"Click an article to the left to load it"}
                    </BBText>
                </BBCol>
            </BBRow>
        </BBContainer>
    }
}
