use crate::{
    api,
    logging::{self, log_data, log_error},
    types::{Alert, Auth0User, Course, Tag, User, Article},
    utils::cookies::{load_cookie, save_cookie},
};
use eyre::Result;
use std::collections::HashMap;
use ycl::foundations::states::BBLoadingState;
use yew::AttrValue;
use yewdux::prelude::*;

static STATE_COOKIE_KEY: &str = "auth_state";
static TOKEN_COOKIE_KEY: &str = "auth_token";
static STATE_COOKIE_MAX_LIFE: u32 = 60 * 5;

#[derive(Store, Default, Clone, PartialEq)]
pub struct MainStore {
    pub courses: HashMap<i64, Course>,
    pub courses_loaded: BBLoadingState,
    pub user: User,
    pub alert: Alert,
    pub auth_loaded: BBLoadingState,
    pub tags: HashMap<i64, Tag>,
    pub articles: HashMap<i64, Article>,
}

impl MainStore {
    pub fn logged_in(&self) -> bool {
        self.user.token.is_some()
    }
}

pub async fn load_all_data(dispatch: Dispatch<MainStore>) {
    dispatch
        .reduce_mut_future(|store| {
            Box::pin(async move {
                store.courses_loaded = BBLoadingState::Loading;

                match api::get_all_data(store.user.token.clone(), store.user.role).await {
                    Ok(data) => {
                        for course in data.courses {
                            store.courses.insert(course.id, course);
                        }

                        for tag in data.tags {
                            store.tags.insert(tag.id, tag);
                        }

                        for article in data.articles {
                            store.articles.insert(article.id, article);
                        }

                        store.courses_loaded = BBLoadingState::Loaded
                    }
                    Err(error) => {
                        gloo::console::error!("Error getting courses:", error.to_string());
                        store.alert.message = Some("There was an error getting courses".into());
                    }
                }
            })
        })
        .await;
}

pub async fn login_from_redirect(dispatch: Dispatch<MainStore>) {
    dispatch
        .reduce_mut_future(|store| {
            Box::pin(async move {
                store.auth_loaded = BBLoadingState::Loading;
                let Ok(url) = gloo::utils::window().location().href() else { return;};
                let Ok(Some(saved_state)) = load_cookie(STATE_COOKIE_KEY) else {return;};
                let Ok(parsed_url) = url::Url::parse(&url)  else {return;};
                let Some(fragment) = parsed_url.fragment() else { return; };
                let url_encoded =
                    url::form_urlencoded::parse(fragment.as_bytes()).collect::<HashMap<_, _>>();
                let Some(access_token )= url_encoded
                    .get("access_token")
                    .map(ToString::to_string)
                     else { 
                        gloo::console::error!("missing access token in redirect uri");
                        return;
                    };
                let Some(url_state )= url_encoded
                    .get("state")
                    .map(ToString::to_string)
                     else { return; };

                if saved_state != url_state {
                    return;
                }

                if let Err(error) =
                    save_cookie(TOKEN_COOKIE_KEY, &access_token, STATE_COOKIE_MAX_LIFE)
                {
                    log_error("Error saving token to cookie", &error);
                }

                let Auth0User {
                    sub,
                    nickname,
                    name,
                    picture,
                    email,
                    email_verified,
                    metadata,
                    updated_at: _updated_at,
                } = api::auth::get_user_info(&access_token).await.unwrap();

                store.user = User {
                    role: metadata.role.into(),
                    id: Some(sub.into()),
                    nickname: Some(nickname.into()),
                    name: Some(name.into()),
                    picture: Some(picture.into()),
                    email: Some(email.into()),
                    email_verified: Some(email_verified),
                    token: Some(access_token.into()),
                };

                store.auth_loaded = BBLoadingState::Loaded;
            })
        })
        .await
}

pub async fn login_from_refresh(dispatch: Dispatch<MainStore>) {
    dispatch
        .reduce_mut_future(|store| {
            Box::pin(async move {
                store.auth_loaded = BBLoadingState::Loading;
                let Ok(Some(token)) = load_cookie(TOKEN_COOKIE_KEY) else {
                    store.auth_loaded = BBLoadingState::Loaded;

                    return;
                };
                let Ok(Auth0User { sub, nickname, name, picture, updated_at: _updated_at, email, email_verified, metadata }) = api::auth::get_user_info(&token).await else {
                    store.auth_loaded = BBLoadingState::Loaded;
                    return
                };

                store.user = User { role: metadata.role.into(), token: Some(token.into()), id: Some(sub.into()), nickname: Some(nickname.into()), name: Some(name.into()), picture: Some(picture.into()), email: Some(email.into()), email_verified: Some(email_verified) };

                store.auth_loaded = BBLoadingState::Loaded;
            })
        })
        .await
}

pub async fn insert_tag(dispatch: Dispatch<MainStore>, name: AttrValue) {
    dispatch
        .reduce_mut_future(|store| {
            Box::pin(async move {
                let Some(token) = store.user.token.clone() else {
                    store.alert.message = Some("Could not create token".into());
                    gloo::console::error!("Token missing when trying to create tag");

                    return;
                };

                match api::insert_tag(&token, name).await {
                    Ok(tag) => {
                        store.alert.message = Some("Tag created".into());
                        store.tags.insert(tag.id, tag);
                    }
                    Err(error) => {
                        store.alert.message = Some("There was an error creating the tag".into());
                        gloo::console::error!(error.to_string());
                    }
                }
            })
        })
        .await
}

pub fn set_alert(dispatch: Dispatch<MainStore>, message: AttrValue) {
    dispatch.reduce_mut(move |store| store.alert.message = Some(message));
}

pub fn reset_alert(dispatch: Dispatch<MainStore>) {
    dispatch.reduce_mut(|store| store.alert.message = None);
}

pub fn error_alert(dispatch: Dispatch<MainStore>, message: impl Into<AttrValue>) {
    dispatch.reduce_mut(|store| store.alert.error(message.into()));
}

pub async fn insert_course(dispatch: Dispatch<MainStore>,
    long_description: AttrValue,
    title: AttrValue,
    tag_id: i64,
    short_description: AttrValue,) {
    dispatch.clone().reduce_mut_future(move |store| {
        Box::pin(async move {
            let Some(token) = store.user.token.clone() else {return};
            match api::insert_course(token, long_description, title, tag_id, short_description).await {
                Ok(course) => {
                    store.courses.insert(course.id, course);
                    store.alert.message = Some("course created".into());
                },
                Err(error) => {
                    store.alert.message = Some("There was an error creating the course".into());
                    gloo::console::error!("Error creating course:", error.to_string());
                },
            }
        })
    }).await
}

pub async fn insert_article(dispatch: Dispatch<MainStore>, title: AttrValue, content: AttrValue) {
    dispatch.clone().reduce_mut_future(|store| Box::pin(async move {
        let Some(token) = store.user.token.clone() else {
            store.alert.error("Error: missing token, please log in and try again");
            return
        };

        match api::insert_article(token, title, content).await {
            Ok(article) => {
                store.articles.insert(article.id, article);
                store.alert.success("Article created");
            }
            Err(error) => {
                store.alert.error("Error Creating article");
                gloo::console::error!("Error creating article:", error.to_string());
            },
        }
    })).await
}

pub fn add_article_to_course(dispatch: Dispatch<MainStore>, article: Article, course_id: i64) {
    dispatch.reduce_mut(|store| {
        let Some(course) = store.courses.get_mut(&course_id) else {
            store.alert.error("Could not find store to add article to");
            return;
        };

        course.articles.push(article);
        course.articles_dirty = true;
        store.alert.success("Article added to course");
    });
}

pub fn remove_article_from_course(dispatch: Dispatch<MainStore>, article_id: i64, course_id: i64) {
    dispatch.reduce_mut(|store| {
        let Some(course) = store.courses.get_mut(&course_id) else {
            store.alert.error("Could not find course to remove article from");
            return;
        };

        course.articles.retain(|article| article.id != article_id);
        course.articles_dirty = true;
        store.alert.success("Article removed from course");
    });
}

pub async fn save_course_articles(dispatch: Dispatch<MainStore>, course_id: i64) {
    dispatch.reduce_mut_future(|store| Box::pin(async move {
        let Some(course) = store.courses.get_mut(&course_id) else {
            store.alert.error("Could not find course to save articles");
            return;
        };

        if !course.articles_dirty {
            store.alert.error("Articles not changed, not saving");
            return;
        }

        let Some(token) = store.user.token.clone() else {
            store.alert.error("Must be logged in to save course articles");
            return;
        };

        if let Err(error) = api::set_course_articles(token, course_id, &course.articles).await {
            store.alert.error("Error saving course articles");
            gloo::console::error!("Error saving course articles:", error.to_string());
            return;
        }

        store.alert.success("Course articles saved");
        course.articles_dirty = false;
    })).await
}
