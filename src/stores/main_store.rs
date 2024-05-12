use crate::{
    api::{self},
    types::{Alert, Article, Auth0User, Course, DbUser, Tag, User},
    utils::cookies::{load_cookie, save_cookie},
};
use dotenvy_macro::dotenv;
use gloo::console::error;
use std::{collections::HashMap, ops::Deref};
use ycl::foundations::states::BBLoadingState;
use yew::AttrValue;
use yewdux::prelude::*;

static STATE_COOKIE_KEY: &str = "auth_state";
static TOKEN_COOKIE_KEY: &str = "auth_token";
static AUTH0_DOMAIN: &str = dotenv!("AUTH0_DOMAIN");
static AUTH0_LOGOUT_REDIRECT: &str = dotenv!("LOGOUT_REDIRECT");
static AUTH0_CLIENT_ID: &str = dotenv!("AUTH0_CLIENT_ID");
static TOKEN_COOKIE_MAX_LIFE: u32 = 60 * 60 * 24;

#[derive(Store, Default, Clone, PartialEq)]
pub struct MainStore {
    pub courses: HashMap<i64, Course>,
    pub courses_loaded: BBLoadingState,
    pub user: User,
    pub alert: Alert,
    pub auth_loaded: BBLoadingState,
    pub tags: HashMap<i64, Tag>,
    pub articles: HashMap<i64, Article>,
    pub preview_articles_by_course: HashMap<i64, Vec<i64>>,
    pub db_user: Option<DbUser>,
}

impl MainStore {
    pub fn logged_in(&self) -> bool {
        self.user.token.is_some()
    }

    pub fn logout_url(&self) -> AttrValue {
        AttrValue::from(format!(
            "{AUTH0_DOMAIN}/v2/logout?returnTo={AUTH0_LOGOUT_REDIRECT}&client_id={AUTH0_CLIENT_ID}"
        ))
    }

    pub fn own_course(&self, course_id: i64) -> bool {
        let Some(user) = &self.db_user else {
            return false;
        };
        user.purchased_courses.contains(&course_id)
    }

    pub fn get_next_article_for_course(&self, course_id: i64) -> Option<i64> {
        let learner_article_ids = self
            .db_user
            .as_ref()?
            .articles
            .iter()
            .filter_map(|learner_article| {
                if learner_article.completed_at.is_some() {
                    Some(learner_article.article_id)
                } else {
                    None
                }
            })
            .collect::<Vec<i64>>();
        let course = self.courses.get(&course_id)?;
        let article_ids = course
            .articles
            .iter()
            .filter_map(|article| {
                gloo::console::log!("article title:", &article.title.as_str().to_owned());
                if article.content.is_some() {
                    Some(article.id)
                } else {
                    None
                }
            })
            .collect::<Vec<i64>>();
        let mut article_id = *article_ids.last()?;

        for id in article_ids {
            if learner_article_ids.contains(&id) {
                continue;
            }

            article_id = id;
            break;
        }

        Some(article_id)
    }
}

pub async fn load_all_data(dispatch: Dispatch<MainStore>) {
    let store = dispatch.get();

    dispatch.reduce_mut(|store| store.courses_loaded = BBLoadingState::Loading);

    match api::get_all_data(store.user.token.clone(), store.user.role).await {
        Ok(data) => dispatch.reduce_mut(|store| {
            for course in data.courses {
                store.courses.insert(course.id, course);
            }

            for tag in data.tags {
                store.tags.insert(tag.id, tag);
            }

            for article in data.articles {
                store.articles.insert(article.id, article);
            }

            store.courses_loaded = BBLoadingState::Loaded;
            store.preview_articles_by_course = data.preview_articles_by_course;
            store.db_user = data.db_user;
        }),
        Err(error) => {
            gloo::console::error!("Error getting courses:", error.to_string());
            dispatch.reduce_mut(|store| {
                store.alert.message = Some("There was an error getting courses".into())
            });
        }
    }
}

pub async fn login_from_redirect(dispatch: Dispatch<MainStore>) {
    dispatch.reduce_mut(|store| store.auth_loaded = BBLoadingState::Loading);

    let Ok(url) = gloo::utils::window().location().href() else {
        dispatch.reduce_mut(|store| store.alert.error("There was a problem handling the Auth0 login/signup. Please try again later and/or let us know in Discord"));
        return;
    };
    let Ok(Some(saved_state)) = load_cookie(STATE_COOKIE_KEY) else {
        dispatch.reduce_mut(|store| store.alert.error("There was a problem handling the Auth0 login/signup. Please try again later and/or let us know in Discord"));
        return;
    };
    let Ok(parsed_url) = url::Url::parse(&url) else {
        dispatch.reduce_mut(|store| store.alert.error("There was a problem handling the Auth0 login/signup. Please try again later and/or let us know in Discord"));
        return;
    };
    let Some(fragment) = parsed_url.fragment() else {
        dispatch.reduce_mut(|store| store.alert.error("There was a problem handling the Auth0 login/signup. Please try again later and/or let us know in Discord"));
        return;
    };
    let url_encoded = url::form_urlencoded::parse(fragment.as_bytes()).collect::<HashMap<_, _>>();
    if let Some(error_description) = url_encoded
        .get("error_description")
        .map(ToString::to_string)
    {
        error!("auth0 returned an error", &error_description);
        dispatch.reduce_mut(|store| store.alert.error(error_description));
        return;
    }
    let Some(access_token) = url_encoded.get("access_token").map(ToString::to_string) else {
        dispatch.reduce_mut(|store| store.alert.error("There was a problem handling the Auth0 login/signup. Please try again later and/or let us know in Discord"));
        return;
    };
    let Some(url_state) = url_encoded.get("state").map(ToString::to_string) else {
        dispatch.reduce_mut(|store| store.alert.error("There was a problem handling the Auth0 login/signup. Please try again later and/or let us know in Discord"));
        return;
    };

    if saved_state != url_state {
        dispatch.reduce_mut(|store| store.alert.error("There was a problem handling the Auth0 login/signup. Please try again later and/or let us know in Discord"));
        return;
    }

    if let Err(_error) = save_cookie(TOKEN_COOKIE_KEY, &access_token, TOKEN_COOKIE_MAX_LIFE) {
        dispatch.reduce_mut(|store| store.alert.error("There was a problem handling the Auth0 login/signup. Please try again later and/or let us know in Discord"));
        return;
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
    } = api::auth::get_user_info(&access_token)
        .await
        .unwrap_or_default();

    dispatch.reduce_mut(|store| {
        store.user = User {
            role: metadata.role.into(),
            id: Some(sub.into()),
            nickname: Some(nickname.into()),
            name: Some(name.into()),
            picture: Some(picture.into()),
            email: Some(email.into()),
            email_verified: Some(email_verified),
            token: Some(access_token.into()),
        }
    });

    dispatch.reduce_mut(|store| store.auth_loaded = BBLoadingState::Loaded);
}

pub async fn login_from_refresh(dispatch: Dispatch<MainStore>) {
    dispatch.reduce_mut(|store| store.auth_loaded = BBLoadingState::Loading);

    let Ok(Some(token)) = load_cookie(TOKEN_COOKIE_KEY) else {
        dispatch.reduce_mut(|store| store.auth_loaded = BBLoadingState::Loaded);

        return;
    };

    let Ok(Auth0User {
        sub,
        nickname,
        name,
        picture,
        updated_at: _updated_at,
        email,
        email_verified,
        metadata,
    }) = api::auth::get_user_info(&token).await
    else {
        dispatch.reduce_mut(|store| store.auth_loaded = BBLoadingState::Loaded);

        return;
    };

    dispatch.reduce_mut(|store| {
        store.user = User {
            role: metadata.role.into(),
            token: Some(token.into()),
            id: Some(sub.into()),
            nickname: Some(nickname.into()),
            name: Some(name.into()),
            picture: Some(picture.into()),
            email: Some(email.into()),
            email_verified: Some(email_verified),
        };
    });

    dispatch.reduce_mut(|store| store.auth_loaded = BBLoadingState::Loaded);
}

pub async fn insert_tag(dispatch: Dispatch<MainStore>, name: AttrValue) {
    let Some(token) = dispatch.get().user.token.clone() else {
        dispatch.reduce_mut(|store| store.alert.message = Some("Could not create token".into()));
        gloo::console::error!("Token missing when trying to create tag");

        return;
    };

    match api::insert_tag(&token, name.clone()).await {
        Ok(mut tag) => {
            dispatch.reduce_mut(|store| store.alert.message = Some("Tag created".into()));
            tag.name = name;
            dispatch.reduce_mut(|store| store.tags.insert(tag.id, tag));
        }
        Err(error) => {
            dispatch.reduce_mut(|store| {
                store.alert.message = Some("There was an error creating the tag".into())
            });
            gloo::console::error!(error.to_string());
        }
    }
}

pub fn set_alert(dispatch: Dispatch<MainStore>, message: impl Into<AttrValue>) {
    dispatch.reduce_mut(move |store| store.alert.success(message));
}

pub fn reset_alert(dispatch: Dispatch<MainStore>) {
    dispatch.reduce_mut(|store| store.alert.message = None);
}

pub fn error_alert(dispatch: Dispatch<MainStore>, message: impl Into<AttrValue>) {
    dispatch.reduce_mut(|store| store.alert.error(message.into()));
}

pub async fn insert_course(
    dispatch: Dispatch<MainStore>,
    long_description: AttrValue,
    title: AttrValue,
    tag_id: i64,
    short_description: AttrValue,
    live: bool,
) {
    let Some(token) = dispatch.get().user.token.clone() else {
        return;
    };
    match api::insert_course(
        token,
        long_description,
        title,
        tag_id,
        short_description,
        live,
    )
    .await
    {
        Ok(course) => {
            dispatch.reduce_mut(|store| {
                store.courses.insert(course.id, course);
                store.alert.message = Some("course created".into());
            });
        }
        Err(error) => {
            dispatch.reduce_mut(|store| {
                store.alert.message = Some("There was an error creating the course".into())
            });
            gloo::console::error!("Error creating course:", error.to_string());
        }
    }
}

pub async fn insert_article(dispatch: Dispatch<MainStore>, title: AttrValue, content: AttrValue) {
    let Some(token) = dispatch.get().user.token.clone() else {
        dispatch.reduce_mut(|store| {
            store
                .alert
                .error("Error: missing token, please log in and try again")
        });
        return;
    };

    match api::insert_article(token, title, content).await {
        Ok(article) => dispatch.reduce_mut(|store| {
            store.articles.insert(article.id, article);
            store.alert.success("Article created");
        }),
        Err(error) => {
            dispatch.reduce_mut(|store| store.alert.error("Error Creating article"));
            gloo::console::error!("Error creating article:", error.to_string());
        }
    }
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
            store
                .alert
                .error("Could not find course to remove article from");
            return;
        };

        course.articles.retain(|article| article.id != article_id);
        course.articles_dirty = true;
        store.alert.success("Article removed from course");
    });
}

pub async fn save_course_articles(dispatch: Dispatch<MainStore>, course_id: i64) {
    let mut store = dispatch.get().deref().clone();

    let Some(course) = store.courses.get_mut(&course_id) else {
        store.alert.error("Could not find course to save articles");
        return;
    };

    if !course.articles_dirty {
        store.alert.error("Articles not changed, not saving");
        return;
    }

    let Some(token) = store.user.token.clone() else {
        store
            .alert
            .error("Must be logged in to save course articles");
        return;
    };

    if let Err(error) = api::set_course_articles(token, course_id, &course.articles).await {
        store.alert.error("Error saving course articles");
        gloo::console::error!("Error saving course articles:", error.to_string());
        return;
    }

    store.alert.success("Course articles saved");
    course.articles_dirty = false;

    dispatch.set(store);
}

pub fn mark_article_completed(dispatch: Dispatch<MainStore>, article_id: i64) {
    dispatch.reduce_mut(|store| {
        let Some(user) = &mut store.db_user else {
            return;
        };
        user.complete_article(article_id);
    })
}

pub fn mark_article_opened(dispatch: Dispatch<MainStore>, article_id: i64) {
    dispatch.reduce_mut(move |store| {
        let Some(db_user) = &mut store.db_user else {
            return;
        };
        db_user.start_article(article_id);
    });
}

pub fn move_article_before(
    dispatch: Dispatch<MainStore>,
    moving_article_id: i64,
    target_article_id: i64,
    course_id: i64,
) {
    dispatch.reduce_mut(|store| {
        let Some(course) = store.courses.get_mut(&course_id) else {
            gloo::console::warn!("could not find course with id", course_id);
            return;
        };

        course.move_article_before(moving_article_id, target_article_id);
    })
}
