#![allow(non_camel_case_types)]
use std::{collections::HashMap, ops::Deref};

use ycl::{
    elements::{
        button::{BBButton, BBButtonStyle},
        title::{BBTitle, BBTitleLevel},
    },
    foundations::{
        align_text::AlignText,
        column::BBCol,
        container::{BBContainer, BBContainerMargin},
        row::BBRow,
        states::BBLoadingState,
    },
    modules::lists::button_list::{BBButtonList, BBButtonListItem},
};
pub use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::prelude::use_store;

use crate::{
    api, logging,
    logging::log_error,
    router::Routes,
    stores::{
        alerts::{AlertsStore, AlertsStoreBuilder},
        articles::{Article, ArticlesStore},
        auth_store::AuthStore,
        courses_store::CourseStore,
    },
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub course_id: i64,
}

#[function_component(CourseArticles)]
pub fn component(props: &Props) -> Html {
    let (auth_state, _) = use_store::<AuthStore>();
    let (articles_store, articles_dispatch) = use_store::<ArticlesStore>();
    let (_alert_store, alert_dispatch) = use_store::<AlertsStore>();
    let navigator = use_navigator().unwrap();
    let available_articles = use_state(HashMap::<i64, Article>::new);
    let (_course_store, course_dispatch) = use_store::<CourseStore>();
    let assigned_article_titles = use_state(HashMap::<i64, Article>::new);
    let article_titles_loaded = use_state(|| BBLoadingState::Initialized);
    let course_loaded = use_state(|| BBLoadingState::Initialized);
    let assigned_article_titles_loaded = use_state(|| BBLoadingState::Initialized);

    {
        let available_articles = available_articles.clone();
        let auth_state = auth_state;
        let alert_dispatch = alert_dispatch;
        let course_id = props.course_id;
        let course_dispatch = course_dispatch;
        let assigned_article_titles = assigned_article_titles.clone();

        use_effect(move || {
            let result = || {};

            if auth_state.loading {
                return result;
            }

            if !auth_state.is_author() {
                alert_dispatch.reduce_mut(|alert_state| {
                    *alert_state =
                        AlertsStoreBuilder::new_error("Only Authors can manage course articles")
                });
                navigator.push(&Routes::Home);
                return result;
            }

            if assigned_article_titles_loaded.is_loaded()
                && article_titles_loaded.is_loaded()
                && course_loaded.is_loaded()
            {
                return result;
            }

            if *course_loaded == BBLoadingState::Initialized {
                let course_dispatch = course_dispatch.clone();
                let alert_dispatch = alert_dispatch.clone();
                let course_loaded = course_loaded.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    course_loaded.clone().set(BBLoadingState::Loading);
                    match api::courses::get_by_id(course_id).await {
                        Ok(course) => course_dispatch.reduce_mut(|courses_state| {
                            courses_state.courses.insert(course.id, course);
                            course_loaded.set(BBLoadingState::Loaded);
                        }),
                        Err(error) => {
                            log_error("error getting course", &error);
                            alert_dispatch.clone().reduce_mut(|alert_state| {
                                *alert_state = AlertsStoreBuilder::new_error(
                                    "There was a problem loading the course",
                                )
                            });
                        }
                    }
                });
            }

            if *article_titles_loaded == BBLoadingState::Initialized {
                logging::log("loading article titles");
                let alert_dispatch = alert_dispatch.clone();
                let token = auth_state.access_token.clone().unwrap_or_default();
                let article_titles_loaded = article_titles_loaded.clone();
                let available_articles = available_articles.clone();
                if token.is_empty() {
                    return result;
                }
                logging::log("really loading article titles");

                wasm_bindgen_futures::spawn_local(async move {
                    article_titles_loaded.clone().set(BBLoadingState::Loading);
                    match api::articles::get_article_titles(token).await {
                        Ok(articles) => {
                            articles_dispatch.reduce_mut(|articles_state| {
                                *articles_state = articles.clone();
                            });
                            available_articles.set(articles.articles);
                            article_titles_loaded.set(BBLoadingState::Loaded);
                        }
                        Err(error) => {
                            log_error("Error getting article titles", &error);
                            alert_dispatch.reduce_mut(|alert_state| {
                                *alert_state =
                                    AlertsStoreBuilder::new_error("Error getting article titles")
                            });
                        }
                    }
                });
            }

            if *assigned_article_titles_loaded == BBLoadingState::Initialized
                && course_loaded.is_loaded()
                && article_titles_loaded.is_loaded()
            {
                logging::log("assigning article titles");

                let assigned_articles = assigned_article_titles.deref().clone();
                let available_articles_clone = available_articles.deref().clone();
                assigned_article_titles.set(assigned_articles);
                available_articles.set(available_articles_clone);
                assigned_article_titles_loaded.set(BBLoadingState::Loaded);
            }

            // if *loaded && !*really_loaded {
            //     let course_article_ids = if let Some(store_course) = course_store
            //         .courses
            //         .iter()
            //         .rfind(|course| course.id == course_id)
            //     {
            //         // remove articles from available and put into assigned
            //         store_course.article_ids.clone()
            //     } else {
            //         alert_dispatch.clone().reduce_mut(|alert_state| {
            //             *alert_state = AlertsStoreBuilder::new_error(
            //                 "Could not find the course we are adding articles to",
            //             )
            //         });
            //         return result;
            //     };

            //     let assigned_articles = available_articles
            //         .iter()
            //         .filter(move |available_article| {
            //             course_article_ids.contains(&available_article.id)
            //         })
            //         .map(ToOwned::to_owned)
            //         .collect::<Vec<Article>>();

            //     assigned_article_titles.set(assigned_articles);

            //     really_loaded.set(true);
            //     return result;
            // }

            // if auth_state.clone().loading {
            //     return result;
            // }

            // if !auth_state.is_author() {
            //     alert_dispatch.clone().reduce_mut(|alert_state| {
            //         *alert_state = AlertsStoreBuilder::new_error(
            //             "You must be an author to assign articles to courses",
            //         )
            //     });
            //     navigator.push(&Routes::Home);
            //     return result;
            // }

            // let token = auth_state.access_token.clone().unwrap_or_default();
            // let alert_dispatch = alert_dispatch.clone();
            // let articles_dispatch = articles_dispatch.clone();
            // let available_articles = available_articles.clone();
            // let course_id = course_id.clone();
            // let course_dispatch = course_dispatch.clone();
            // let assigned_article_titles = assigned_article_titles.clone();

            // wasm_bindgen_futures::spawn_local(async move {
            //     match api::courses::get_by_id(course_id).await {
            //         Ok(course) => {
            //             course_dispatch.reduce_mut(|course_state| {
            //                 if let Some((index, store_course)) = course_state
            //                     .courses
            //                     .iter()
            //                     .enumerate()
            //                     .find(|(index, store_course)| store_course.id == course.id)
            //                 {
            //                     course_state.courses.remove(index);
            //                 }

            //                 course_state.courses.push(course);
            //             });
            //         }
            //         Err(error) => {
            //             log_error("error getting course", &error);
            //             alert_dispatch.clone().reduce_mut(|alert_state| {
            //                 *alert_state = AlertsStoreBuilder::new_error(
            //                     "There was a problem loading the course",
            //                 )
            //             });
            //         }
            //     }

            //     match api::articles::get_article_titles(token).await {
            //         Ok(articles) => {
            //             articles_dispatch.reduce_mut(|articles_state| {
            //                 *articles_state = articles.clone();
            //             });
            //             available_articles.set(articles.articles);
            //         }
            //         Err(error) => {
            //             log_error("Error getting article titles", &error);
            //             alert_dispatch.reduce_mut(|alert_state| {
            //                 *alert_state =
            //                     AlertsStoreBuilder::new_error("Error getting article titles")
            //             });
            //         }
            //     }

            //     loaded.set(true);
            // });

            result
        });
    }

    let all_articles_onclick = {
        let assigned_article_titles = assigned_article_titles.clone();
        let articles_store = articles_store;
        let available_articles = available_articles.clone();

        Callback::from(move |id: AttrValue| {
            let id = id.to_string().parse::<i64>().unwrap();
            let article = articles_store.articles.get(&id);
            let mut assigned_articles = assigned_article_titles.deref().clone();
            if let Some(article) = article {
                assigned_articles.insert(id, article.clone());
            }
            assigned_article_titles.set(assigned_articles);

            let mut available_articles_clone = available_articles.deref().clone();
            available_articles_clone.remove(&id);
            available_articles.set(available_articles_clone);
        })
    };

    let assigned_articles_onclick = {
        let assigned_article_titles = assigned_article_titles.clone();
        let available_articles = available_articles.clone();

        Callback::from(move |id: AttrValue| {
            let id = id
                .to_string()
                .parse::<i64>()
                .expect("assigned article id is not a number");
            let mut assigned_articles = assigned_article_titles.deref().clone();
            let mut available_articles_clone = available_articles.deref().clone();

            if let Some(article) = assigned_articles.remove(&id) {
                available_articles_clone.insert(id, article);
            }

            assigned_article_titles.set(assigned_articles);
            available_articles.set(available_articles_clone);
        })
    };

    let save_onclick = { Callback::from(move |_| {}) };

    html! {
        <BBContainer margin={BBContainerMargin::Normal}>
            <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>
                {"Course Articles"}
            </BBTitle>
            <BBRow>
                <BBCol>
                    <BBTitle level={BBTitleLevel::Two} align={AlignText::Center}>
                        {"Assigned"}
                    </BBTitle>
                    <BBButtonList items={extract_article_titles(&assigned_article_titles)} onclick={assigned_articles_onclick} />
                </BBCol>
                <BBCol>
                    <BBTitle level={BBTitleLevel::Two} align={AlignText::Center}>
                        {"All Articles"}
                    </BBTitle>
                    <BBButtonList items={extract_article_titles(&available_articles)} onclick={all_articles_onclick} />
                </BBCol>
            </BBRow>
            <BBRow>
                <BBButton button_style={BBButtonStyle::PrimaryLight} onclick={save_onclick}>{"Save"}</BBButton>
            </BBRow>
        </BBContainer>
    }
}

fn extract_article_titles(titles: &HashMap<i64, Article>) -> Vec<BBButtonListItem> {
    titles
        .iter()
        .map(|(_id, title)| BBButtonListItem {
            label: AttrValue::from(title.title.clone()),
            id: AttrValue::from(title.id.to_string()),
        })
        .collect()
}
