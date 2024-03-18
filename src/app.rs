use crate::{
    auth::auth_login_uri,
    components::alert::Alert,
    logging::log_error,
    router::{switch, Routes},
    stores::main_store::{self, MainStore},
    utils::cookies::delete_cookie,
};
use dotenvy_macro::dotenv;
use gloo::console::error;
use ycl::{
    foundations::{roles::BBRole, states::BBLoadingState},
    modules::{
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

#[function_component(App)]
pub fn component() -> Html {
    let (store, dispatch) = use_store::<MainStore>();

    {
        let store = store.clone();
        let dispatch = dispatch.clone();

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
        let store = store.clone();
        let dispatch = dispatch.clone();

        Callback::from(move |_event: ()| {
            if let Err(error) = delete_cookie("auth_token") {
                main_store::error_alert(dispatch.clone(), "Error logging out");
                log_error("error deleting token cookie", &error);
            }

            if let Err(error) = gloo::utils::window()
                .location()
                .set_href(store.logout_url().as_str())
            {
                main_store::error_alert(dispatch.clone(), "Error logging out");
                gloo::console::error!("Error navigating to logout uri:", error);
            }
        })
    };

    let onclick_login = {
        let dispatch = dispatch.clone();

        Callback::from(move |_| {
            let login_uri = match auth_login_uri() {
                Ok(uri) => uri,
                Err(error) => {
                    gloo::console::error!("Error getting login uri:", error.to_string());
                    // main_store::error_alert(dispatch, "Error calculating the login uri");
                    "/".into()
                }
            };

            gloo::utils::window()
                .location()
                .set_href(login_uri.as_str())
                .expect("error navigating to login url");
        })
    };

    html! {
        <BrowserRouter>
            <Alert />
            <BBNavbar<Routes>
                is_authenticated={store.logged_in()}
                links={create_routes(store.user.role)}
                show_brand={true}
                username={store.user.nickname.clone()}
                logout_url={store.logout_url()}
                {logout_onclick}
                role={store.user.role}
                onlogin_click={onclick_login}
            />
            <main role="main">
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
