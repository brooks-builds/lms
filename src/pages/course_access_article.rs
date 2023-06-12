#![allow(non_camel_case_types)]
use ycl::{
    elements::title::{BBTitle, BBTitleLevel},
    foundations::{
        align_text::AlignText,
        column::{BBCol, BBColWidth},
        container::BBContainer,
        row::BBRow,
    },
    modules::course_content::BBCourseContent,
};
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::prelude::use_store;

use crate::{
    components::{course_nav::CourseNav, next_article::NextArticle},
    logging::log_data,
    router::Routes,
    stores::main_store::MainStore,
    utils::nav_article_onclick::{self, article_nav_onclick},
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub course_id: i64,
    pub article_id: i64,
}

#[function_component(CourseAccessArticle)]
pub fn component(props: &Props) -> Html {
    let (store, dispatch) = use_store::<MainStore>();
    let navigation = use_navigator().unwrap();

    if let Some(course) = store.courses.get(&props.course_id) {
        let article_id = props.article_id;
        let course_id = course.id;
        let onclick_purchase = {
            let store = store.clone();
            Callback::from(move |_: ()| {
                if store.logged_in() {
                    navigation.push(&Routes::CoursePurchase { course_id });
                } else {
                    navigation.push(&Routes::Login);
                }
            })
        };
        let preview_articles = store
            .preview_articles_by_course
            .get(&course.id)
            .cloned()
            .unwrap_or_default();
        let next_article_onclick = Callback::from(|completed_article_id: i64| {
            log_data("moving to next article from: ", completed_article_id);
        });

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
                            <CourseNav course_id={props.course_id} {preview_articles} onclick={article_nav_onclick(store.clone(), dispatch)} />
                        </BBCol>
                        <BBCol>
                            <BBCourseContent
                                have_access={false}
                                logged_in={store.logged_in()}
                                course={article.content.clone().unwrap_or_default()}
                                {onclick_purchase}
                            />
                            <NextArticle course_id={props.course_id} article_id={props.article_id} onclick={next_article_onclick} />
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
