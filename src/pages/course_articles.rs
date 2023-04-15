use std::ops::Deref;

use ycl::{
    elements::{
        text::BBText,
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
use yew_hooks::{use_async, use_effect_once};
use yew_router::prelude::use_navigator;
use yewdux::prelude::use_store;

use crate::{
    api,
    database_queries::get_lms_article_titles,
    logging::{log_data, log_error},
    router::Routes,
    stores::{
        alerts::{AlertsStore, AlertsStoreBuilder},
        articles::{Article, ArticlesStore},
        auth_store::AuthStore,
    },
};

#[derive(Properties, PartialEq)]
pub struct Props {}

#[function_component(CourseArticles)]
pub fn component(_props: &Props) -> Html {
    let (auth_state, _) = use_store::<AuthStore>();
    let (articles_store, articles_dispatch) = use_store::<ArticlesStore>();
    let (_alert_store, alert_dispatch) = use_store::<AlertsStore>();
    let navigator = use_navigator().unwrap();

    {
        use_effect(move || {
            let result = || {};

            if auth_state.loading {
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

            wasm_bindgen_futures::spawn_local(async move {
                match api::articles::get_article_titles(token).await {
                    Ok(articles) => {
                        articles_dispatch.reduce_mut(|articles_state| {
                            *articles_state = articles;
                        });
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

            result
        });
    }

    let assigned_article_titles = use_state(|| Vec::<Article>::new());

    let all_articles_onclick = {
        let assigned_article_titles = assigned_article_titles.clone();
        let articles_store = articles_store.clone();

        Callback::from(move |id: AttrValue| {
            let id = id.to_string().parse::<i64>().unwrap();
            let article = articles_store.clone_by_id(id);
            let mut assigned_articles = assigned_article_titles.deref().clone();
            if let Some(article) = article {
                assigned_articles.push(article);
            }
            assigned_article_titles.set(assigned_articles);
        })
    };

    let assigned_articles_onclick = Callback::from(|id: AttrValue| {});

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
                    <BBButtonList items={extract_article_titles(&articles_store.articles)} onclick={all_articles_onclick} />
                </BBCol>
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
