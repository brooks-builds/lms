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
    api,
    components::{course_nav::CourseNav, next_article::NextArticle},
    router::Routes,
    stores::main_store::{self, MainStore},
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

    {
        let dispatch = dispatch.clone();
        let article_id = props.article_id;
        let store = store.clone();

        use_effect(move || {
            let return_closure = || {};
            let dispatch = dispatch.clone();
            let Some(db_user) = &store.db_user else {
                return return_closure;
            };

            if !db_user.has_started_article(article_id) {
                main_store::mark_article_opened(dispatch.clone(), article_id);

                {
                    let store = store.clone();
                    let user_id = db_user.id;

                    wasm_bindgen_futures::spawn_local(async move {
                        let Some(token) = store.user.token.clone() else {
                            return;
                        };
                        if let Err(error) =
                            api::insert_user_article(token, user_id, article_id).await
                        {
                            gloo::console::error!(
                                "Error inserting user article:",
                                error.to_string()
                            );
                            main_store::error_alert(
                                dispatch,
                                "There was an error marking the article as started",
                            );
                        }
                    });
                }
            }

            return_closure
        })
    };

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

        let next_article_onclick = {
            let store = store.clone();
            Callback::from(move |completed_article_id: i64| {
                let Some(user) = &store.db_user else {
                    gloo::console::error!("missing user so cannot mark article read");
                    return;
                };
                let Some(token) = store.user.token.clone() else {
                    gloo::console::error!("missing token so cannot mark article completed");
                    return;
                };
                let user_id = user.id;
                let dispatch = dispatch.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    match api::completed_user_article(token.clone(), user_id, completed_article_id)
                        .await
                    {
                        Ok(_) => {
                            // TODO upsert the completed article in the case it doesn't exist yet: https://hasura.io/docs/latest/mutations/postgres/upsert/
                            main_store::mark_article_completed(
                                dispatch.clone(),
                                completed_article_id,
                            );
                        }
                        Err(error) => {
                            gloo::console::error!(
                                "Error completing user article",
                                error.to_string()
                            );
                            main_store::error_alert(
                                dispatch.clone(),
                                "There was an error marking the article as completed",
                            );
                        }
                    }
                });
            })
        };

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
                            <CourseNav course_id={props.course_id} {preview_articles} />
                        </BBCol>
                        <BBCol width={BBColWidth::Nine}>
                            <BBCourseContent
                                have_access={store.own_course(course.id)}
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
