use std::ops::Deref;

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
    },
    modules::lists::button_list::{BBButtonList, BBButtonListItem},
};
pub use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::prelude::use_store;

use crate::{
    api,
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
    let available_articles = use_state(|| Vec::<Article>::new());
    let loaded = use_state(|| false);
    let really_loaded = use_state(|| false);
    let (course_store, course_dispatch) = use_store::<CourseStore>();
    let assigned_article_titles = use_state(|| Vec::<Article>::new());

    {
        let available_articles = available_articles.clone();
        let auth_state = auth_state.clone();
        let alert_dispatch = alert_dispatch.clone();
        let course_id = props.course_id;
        let course_dispatch = course_dispatch.clone();
        let assigned_article_titles = assigned_article_titles.clone();

        use_effect(move || {
            let result = || {};

            if *loaded && *really_loaded {
                return result;
            }

            if *loaded && !*really_loaded {
                let course_article_ids = if let Some(store_course) = course_store
                    .courses
                    .iter()
                    .rfind(|course| course.id == course_id)
                {
                    // remove articles from available and put into assigned
                    store_course.article_ids.clone()
                } else {
                    alert_dispatch.clone().reduce_mut(|alert_state| {
                        *alert_state = AlertsStoreBuilder::new_error(
                            "Could not find the course we are adding articles to",
                        )
                    });
                    return result;
                };

                let assigned_articles = available_articles
                    .iter()
                    .filter(move |available_article| {
                        course_article_ids.contains(&available_article.id)
                    })
                    .map(ToOwned::to_owned)
                    .collect::<Vec<Article>>();

                assigned_article_titles.set(assigned_articles);

                really_loaded.set(true);
                return result;
            }

            if auth_state.clone().loading {
                return result;
            }

            if !auth_state.is_author() {
                alert_dispatch.clone().reduce_mut(|alert_state| {
                    *alert_state = AlertsStoreBuilder::new_error(
                        "You must be an author to assign articles to courses",
                    )
                });
                navigator.push(&Routes::Home);
                return result;
            }

            let token = auth_state.access_token.clone().unwrap_or_default();
            let alert_dispatch = alert_dispatch.clone();
            let articles_dispatch = articles_dispatch.clone();
            let available_articles = available_articles.clone();
            let course_id = course_id.clone();
            let course_dispatch = course_dispatch.clone();
            let assigned_article_titles = assigned_article_titles.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match api::courses::get_by_id(course_id).await {
                    Ok(course) => {
                        course_dispatch.reduce_mut(|course_state| {
                            if let Some((index, store_course)) = course_state
                                .courses
                                .iter()
                                .enumerate()
                                .find(|(index, store_course)| store_course.id == course.id)
                            {
                                course_state.courses.remove(index);
                            }

                            course_state.courses.push(course);
                        });
                    }
                    Err(error) => {
                        log_error("error getting course", &error);
                        alert_dispatch.clone().reduce_mut(|alert_state| {
                            *alert_state = AlertsStoreBuilder::new_error(
                                "There was a problem loading the course",
                            )
                        });
                    }
                }

                match api::articles::get_article_titles(token).await {
                    Ok(articles) => {
                        articles_dispatch.reduce_mut(|articles_state| {
                            *articles_state = articles.clone();
                        });
                        available_articles.set(articles.articles);
                    }
                    Err(error) => {
                        log_error("Error getting article titles", &error);
                        alert_dispatch.reduce_mut(|alert_state| {
                            *alert_state =
                                AlertsStoreBuilder::new_error("Error getting article titles")
                        });
                    }
                }

                loaded.set(true);
            });

            result
        });
    }

    let all_articles_onclick = {
        let assigned_article_titles = assigned_article_titles.clone();
        let articles_store = articles_store.clone();
        let available_articles = available_articles.clone();

        Callback::from(move |id: AttrValue| {
            let id = id.to_string().parse::<i64>().unwrap();
            let article = articles_store.clone_by_id(id);
            let mut assigned_articles = assigned_article_titles.deref().clone();
            if let Some(article) = article {
                assigned_articles.push(article);
            }
            assigned_article_titles.set(assigned_articles);

            let available_articles_clone = available_articles
                .deref()
                .clone()
                .into_iter()
                .filter(|article| article.id != id)
                .collect::<Vec<Article>>();
            available_articles.set(available_articles_clone);
        })
    };

    let assigned_articles_onclick = Callback::from(|id: AttrValue| {});

    let save_onclick = {
        let assigned_article_titles = assigned_article_titles.clone();
        let course_id = props.course_id;
        let auth_state = auth_state.clone();
        let alert_dispatch = alert_dispatch.clone();

        Callback::from(move |_| {
            let course_id = course_id.clone();
            let articles = assigned_article_titles.clone();
            let auth_state = auth_state.clone();
            let alert_dispatch = alert_dispatch.clone();
            let assigned_article_titles = assigned_article_titles.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let assigned_article_titles = assigned_article_titles.clone();
                let token = auth_state.access_token.clone().unwrap_or_default();
                match api::courses::set_course_articles(course_id, &*assigned_article_titles, token)
                    .await
                {
                    Ok(_result) => {
                        alert_dispatch.reduce_mut(|alert_state| {
                            *alert_state = AlertsStoreBuilder::new()
                                .message("Articles saved to course")
                                .icon(ycl::elements::icon::BBIconType::Star)
                                .alert_type(ycl::modules::banner::BBBannerType::Success)
                                .build()
                                .unwrap()
                        });
                    }
                    Err(error) => {
                        log_error("Erro saving articles to course", &error);
                        alert_dispatch.reduce_mut(|alert_state| {
                            *alert_state =
                                AlertsStoreBuilder::new_error("Error saving articles to course")
                        });
                    }
                }
            });
        })
    };

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
                    <BBButtonList items={extract_article_titles(&*available_articles)} onclick={all_articles_onclick} />
                </BBCol>
            </BBRow>
            <BBRow>
                <BBButton button_style={BBButtonStyle::PrimaryLight} onclick={save_onclick}>{"Save"}</BBButton>
            </BBRow>
        </BBContainer>
    }
}

fn extract_article_titles(titles: &[Article]) -> Vec<BBButtonListItem> {
    titles
        .iter()
        .map(|title| BBButtonListItem {
            label: AttrValue::from(title.title.clone()),
            id: AttrValue::from(title.id.to_string()),
        })
        .collect()
}

struct ArticleTitle {
    pub id: i64,
    pub title: String,
}
