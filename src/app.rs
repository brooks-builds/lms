use ycl::modules::{
    nav::{
        navbar::BBNavbar,
        navbar_link::{BBNavbarLink, BBNavbarLinkBuilder},
    },
    site_footer::BBSiteFooter,
};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::use_store;

use crate::{
    components::alert::Alert,
    router::{switch, Routes},
    stores::auth_store::AuthStore,
};

#[function_component(App)]
pub fn component() -> Html {
    let (auth_store, _auth_dispatch) = use_store::<AuthStore>();

    html! {
        <BrowserRouter>
            <Alert />
            <BBNavbar<Routes>
                create_account_route={Routes::CreateAccount}
                is_authenticated={auth_store.logged_in}
                links={create_routes()}
                login_route={Routes::Login}
                show_brand={true}
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
