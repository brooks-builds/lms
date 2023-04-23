use ycl::{
    elements::title::{BBTitle, BBTitleLevel},
    foundations::{
        align_text::AlignText,
        column::BBCol,
        container::{BBContainer, BBContainerMargin},
        row::BBRow,
        states::BBLoadingState,
    },
    modules::nav::course_nav::{BBCourseNav, BBCourseNavArticleBuilder},
};
use yew::{function_component, html, use_effect, use_state, Html, Properties};
use yewdux::prelude::use_store;

use crate::{
    api,
    logging::log_error,
    router::Routes,
    stores::{
        alerts::{AlertsStore, AlertsStoreBuilder},
        courses_store::{self, CourseStore},
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

    {
        let alert_dispatch = alert_dispatch.clone();

        use_effect(move || {
            let result = || {};
            let alert_dispatch = alert_dispatch.clone();
            let courses_dispatch = courses_dispatch.clone();

            if *course_loading_state == BBLoadingState::Initialized {
                wasm_bindgen_futures::spawn_local(async move {
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

    let articles = vec![];

    html! {
        <BBContainer margin={BBContainerMargin::Normal}>
            <BBRow>
                <BBCol>
                    <BBCourseNav<Routes> {articles} />
                </BBCol>
                <BBCol>
                    <BBTitle align={AlignText::Center} level={BBTitleLevel::One}>{&course.name}</BBTitle>
                </BBCol>
            </BBRow>
        </BBContainer>
    }
}
