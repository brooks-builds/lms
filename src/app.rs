use ycl::{
    elements::icon::BBIconType,
    foundations::{roles::BBRole, states::BBLoadingState},
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
use yew_router::prelude::*;
use yewdux::prelude::use_store;

use crate::{
    components::alert::Alert,
    logging::log_error,
    router::{switch, Routes},
    stores::{
        alerts::{AlertsStore, AlertsStoreBuilder},
        auth_store::{logout_url, AuthStore},
        main_store::{self, MainStore},
    },
    utils::cookies::delete_cookie,
};

#[function_component(App)]
pub fn component() -> Html {
    let (store, dispatch) = use_store::<MainStore>();
    let (auth_store, _auth_dispatch) = use_store::<AuthStore>();
    let (_, alert_dispatch) = use_store::<AlertsStore>();

    {
        let store = store.clone();
        use_effect(move || {
            let returning = || {};

            match store.auth_loaded {
                BBLoadingState::Initialized => {
                    let dispatch = dispatch.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        main_store::login_from_refresh(dispatch.clone()).await;
                    });
                }
                BBLoadingState::Loading => {}
                BBLoadingState::Loaded => {
                    if store.courses_loaded == BBLoadingState::Initialized {
                        wasm_bindgen_futures::spawn_local(async move {
                            main_store::load_all_data(dispatch).await;
                        });
                    }
                }
            }

            returning
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
                is_authenticated={store.logged_in}
                links={create_routes(store.user.role)}
                login_route={Routes::Login}
                show_brand={true}
                username={store.user.nickname.clone()}
                logout_url={logout_url()}
                {logout_onclick}
                role={store.user.role}
            />
            <main>
                <Switch<Routes> render={switch} />
            </main>
            <BBSiteFooter<Routes>
                left_links={create_routes(store.user.role)}
                right_links={vec![]}
            />
        </BrowserRouter>
    }
}

fn create_routes(role: BBRole) -> Vec<BBNavbarLink<Routes>> {
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
        BBRole::Public => (),
    }

    routes
}
