use crate::{
    api,
    logging::log_error,
    types::{Alert, Auth0User, Course, User},
    utils::cookies::{load_cookie, save_cookie},
};
use eyre::Result;
use std::collections::HashMap;
use ycl::foundations::states::BBLoadingState;
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
    pub logged_in: bool,
}

pub async fn load_all_courses(dispatch: Dispatch<MainStore>) {
    dispatch
        .reduce_mut_future(|store| {
            Box::pin(async move {
                store.courses_loaded = BBLoadingState::Loading;

                match api::courses::get_all_courses(store.user.token.clone(), store.user.role).await
                {
                    Ok(courses) => {
                        for course in courses {
                            store.courses.insert(course.id, course);
                            store.courses_loaded = BBLoadingState::Loaded
                        }
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
                let Ok(url) = gloo::utils::window().location().href() else { return;};
                let Ok(Some(saved_state)) = load_cookie(STATE_COOKIE_KEY) else {return;};
                let Ok(parsed_url) = url::Url::parse(&url)  else {return;};
                let Some(fragment) = parsed_url.fragment() else { return; };
                let url_encoded =
                    url::form_urlencoded::parse(fragment.as_bytes()).collect::<HashMap<_, _>>();
                let Some(access_token )= url_encoded
                    .get("access_token")
                    .map(ToString::to_string)
                     else { return ;};
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

                store.user.token = Some(access_token.clone().into());

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
                    ..store.user.clone()
                };

                store.logged_in = true;
            })
        })
        .await
}
