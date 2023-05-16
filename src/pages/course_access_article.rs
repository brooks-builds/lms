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
    components::course_nav::CourseNav,
    stores::{
        alerts::AlertsStore,
        articles::{Article, ArticlesStore},
        courses_store::{CourseStore, StoreCourse},
        main_store::MainStore,
    },
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub course_id: i64,
    pub article_id: i64,
}

#[function_component(CourseAccessArticle)]
pub fn component(props: &Props) -> Html {
    let (store, dispatch) = use_store::<MainStore>();
    let (store_course, store_dispatch) = use_store::<CourseStore>();
    let (_, alert_dispatch) = use_store::<AlertsStore>();
    let course_loaded_flag = use_state(|| BBLoadingState::Initialized);
    let course: UseStateHandle<Option<StoreCourse>> = use_state(|| None);
    let articles_loaded_flag = use_state(|| BBLoadingState::Initialized);
    let (_, articles_dispatch) = use_store::<ArticlesStore>();
    let article_state: UseStateHandle<Option<Article>> = use_state(|| None);

    if let Some(course) = store.courses.get(&props.course_id) {
        let article_id = props.article_id;
        if let Some(article) = course
            .articles
            .iter()
            .find(move |article| article.id == article_id)
        {
            html! {
                <BBContainer>
                    <BBTitle align={AlignText::Center} level={BBTitleLevel::One}>
                        {course.title.clone()}
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
