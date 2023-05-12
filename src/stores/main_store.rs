use crate::{
    api,
    logging::{self, log_data, log_error},
    types::{Alert, Auth0User, Course, Tag, User},
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

                        store.courses_loaded = BBLoadingState::Loaded
                    }
                    Err(error) => {
                        gloo::console::error!("Error getting courses:", error.to_string());
                        store.alert.message = "There was an error getting courses".into();
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
                let Ok(Some(token)) = load_cookie(TOKEN_COOKIE_KEY) else {return;};
                let Ok(Auth0User { sub, nickname, name, picture, updated_at: _updated_at, email, email_verified, metadata }) = api::auth::get_user_info(&token).await else {
                    return};

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
                    store.alert.message = "Could not create token".into();
                    gloo::console::error!("Token missing when trying to create tag");

                    return;
                };

                match api::insert_tag(&token, name).await {
                    Ok(tag) => {
                        store.alert.message = "Tag created".into();
                        store.tags.insert(tag.id, tag);
                    }
                    Err(error) => {
                        store.alert.message = "There was an error creating the tag".into();
                        gloo::console::error!(error.to_string());
                    }
                }
            })
        })
        .await
}

pub fn set_alert(dispatch: Dispatch<MainStore>, message: AttrValue) {
    dispatch.reduce_mut(move |store| store.alert.message = message);
}

pub async fn insert_course(dispatch: Dispatch<MainStore>,
    long_description: AttrValue,
    title: AttrValue,
    tag_id: i64,
    short_description: AttrValue,) {
    dispatch.reduce_mut_future(|store| {
        Box::pin(async move {
            let Some(token) = store.user.token.clone() else {return};
            match api::insert_course(token, long_description, title, tag_id, short_description).await {
                Ok(course) => {store.courses.insert(course.id, course);},
                Err(error) => {
                    store.alert.message = "There was an error creating the course".into();
                    gloo::console::error!("Error creating course:", error.to_string());
                },
            }
        })
    }).await
}
