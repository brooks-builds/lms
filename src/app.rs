use ycl::{
    elements::icon::BBIconType,
    foundations::roles::BBRole,
    modules::{
        banner::BBBannerType,
        nav::{
            navbar::BBNavbar,
            navbar_link::{BBNavbarLink, BBNavbarLinkBuilder},
        },
        site_footer::BBSiteFooter,
    },
};
use yew::prelude::*;
use yew_hooks::use_effect_once;
use yew_router::prelude::*;
use yewdux::prelude::use_store;

use crate::{
    api::auth::get_userinfo,
    components::alert::Alert,
    logging::log_error,
    router::{switch, Routes},
    stores::{
        alerts::{AlertsStore, AlertsStoreBuilder},
        auth_store::{logout_url, AuthStore},
    },
    utils::cookies::{delete_cookie, load_cookie},
};

#[function_component(App)]
pub fn component() -> Html {
    let (auth_store, auth_dispatch) = use_store::<AuthStore>();
    let (_, alert_dispatch) = use_store::<AlertsStore>();

    {
        let alert_dispatch = alert_dispatch.clone();
        use_effect_once(move || {
            let token = match load_cookie("auth_token") {
                Ok(token) => token,
                Err(error) => {
                    log_error("Error loading token on app boot", &error);
                    alert_dispatch.reduce_mut(|alert_store| {
                        *alert_store = AlertsStoreBuilder::new()
                            .icon(BBIconType::Warning)
                            .message("Error logging in, please try logging in again")
                            .alert_type(BBBannerType::Error)
                            .build()
                            .unwrap()
                    });
                    None
                }
            };

            wasm_bindgen_futures::spawn_local(async move {
                if let Some(token) = token {
                    auth_dispatch
                    .clone()
                    .reduce_mut_future(|auth_state| {
                        Box::pin(async move {
                            match get_userinfo(&token).await {
                                Ok(userinfo) => {
                                    auth_state.nickname = Some(userinfo.nickname);
                                    auth_state.roles = userinfo.brooks_builds.roles;
                                    auth_state.logged_in = true;
                                    auth_state.access_token = Some(token.clone());
                                }
                                Err(error) => {
                                    log_error("Error getting user info", &error);
                                    alert_dispatch.reduce_mut(|alert_store| {
                                        *alert_store = AlertsStoreBuilder::new()
                                            .icon(BBIconType::Warning)
                                            .message("Error logging in from cookie, please try logging in again")
                                            .alert_type(BBBannerType::Error)
                                            .build()
                                            .unwrap()
                                    })
                                }
                            };
                        })
                    })
                    .await
                }

                auth_dispatch.reduce_mut(|auth_state| auth_state.loading = false);
            });

            || ()
        });
    }

    let logout_onclick = {
        Callback::from(move |_event: ()| {
            if let Err(error) = delete_cookie("auth_token") {
                alert_dispatch.reduce_mut(|alert_store| {
                    *alert_store = AlertsStoreBuilder::new()
                        .icon(BBIconType::Warning)
                        .message("Error logging out, please try again")
                        .alert_type(BBBannerType::Error)
                        .build()
                        .unwrap()
                });
                log_error("error deleting token cookie", &error);
            }

            if let Err(_error) = gloo::utils::window().location().set_href(&logout_url()) {
                alert_dispatch.reduce_mut(|alert_store| {
                    *alert_store = AlertsStoreBuilder::new()
                        .icon(BBIconType::Warning)
                        .message("Error logging out, please try again")
                        .alert_type(BBBannerType::Error)
                        .build()
                        .unwrap()
                });
            }
        })
    };

    html! {
        <BrowserRouter>
            <Alert />
            <BBNavbar<Routes>
                create_account_route={Routes::CreateAccount}
                is_authenticated={auth_store.logged_in}
                links={create_routes(&auth_store.roles)}
                login_route={Routes::Login}
                show_brand={true}
                username={auth_store.nickname.clone()}
                logout_url={logout_url()}
                {logout_onclick}
                roles={auth_store.roles.clone()}
            />
            <main>
                <Switch<Routes> render={switch} />
            </main>
            <BBSiteFooter<Routes>
                left_links={create_routes(&auth_store.roles)}
                right_links={vec![]}
            />
        </BrowserRouter>
    }
}

fn create_routes(roles: &[BBRole]) -> Vec<BBNavbarLink<Routes>> {
    let mut routes = vec![
        BBNavbarLinkBuilder::new()
            .to(Routes::Home)
            .label("Home")
            .build()
            .unwrap(),
        BBNavbarLinkBuilder::new()
            .to(Routes::Courses)
            .label("Courses")
            .build()
            .unwrap(),
    ];

    for role in roles {
        match role {
            BBRole::Author => {
                routes.extend([
                    BBNavbarLinkBuilder::new()
                        .to(Routes::Tags)
                        .label("Tags")
                        .build()
                        .unwrap(),
                    BBNavbarLinkBuilder::new()
                        .to(Routes::CreateCourse)
                        .label("Create Course")
                        .build()
                        .unwrap(),
                    BBNavbarLinkBuilder::new()
                        .to(Routes::CreateArticle)
                        .label("Create Article")
                        .build()
                        .unwrap(),
                ]);
            }
            BBRole::Learner => (),
        }
    }
    routes
}
