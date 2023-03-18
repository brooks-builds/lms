use ycl::{modules::{
    nav::{
        navbar::BBNavbar,
        navbar_link::{BBNavbarLink, BBNavbarLinkBuilder},
    },
    site_footer::BBSiteFooter, banner::BBBannerType,
}, elements::icon::BBIconType};
use yew::prelude::*;
use yew_hooks::use_effect_once;
use yew_router::prelude::*;
use yewdux::prelude::use_store;

use crate::{
    components::alert::Alert,
    router::{switch, Routes},
    stores::{auth_store::AuthStore, alerts::{AlertsStore, AlertsStoreBuilder}}, utils::cookies::load_cookie, logging::log_error, api,
};

#[function_component(App)]
pub fn component() -> Html {
    let (auth_store, auth_dispatch) = use_store::<AuthStore>();
    let (_, alert_dispatch) = use_store::<AlertsStore>();

    use_effect_once(move || {
        if let some(token) = match load_cookie("auth_token") {
            Ok(token) => token,
            Err(error) => {
                log_error("Error loading token on app boot", error);
                alert_dispatch.reduce_mut(|alert_store| *alert_store = AlertsStoreBuilder::new().icon(BBIconType::Warning).message("Error logging in, please try logging in again").alert_type(BBBannerType::Error).build().unwrap());
                None
            } {
                let userinfo = match api::auth::get_userinfo(&token).awai
            }
        };
    })

    html! {
        <BrowserRouter>
            <Alert />
            <BBNavbar<Routes>
                create_account_route={Routes::CreateAccount}
                is_authenticated={auth_store.logged_in}
                links={create_routes()}
                login_route={Routes::Login}
                show_brand={true}
                username={auth_store.nickname.clone()}
            />
            <main>
                <Switch<Routes> render={switch} />
            </main>
            <BBSiteFooter<Routes>
                left_links={create_routes()}
                right_links={vec![]}
            />
        </BrowserRouter>
    }
}

fn create_routes() -> Vec<BBNavbarLink<Routes>> {
    vec![
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
    ]
}
