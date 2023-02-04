use ycl::modules::nav::{
    navbar::BBNavbar,
    navbar_link::{BBNavbarLink, BBNavbarLinkBuilder},
};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::{switch, Routes};

#[function_component(App)]
pub fn component() -> Html {
    html! {
        <BrowserRouter>
            <BBNavbar<Routes>
                create_account_route={Routes::CreateAccount}
                is_authenticated={false}
                links={create_routes()}
                login_route={Routes::Login}
                show_brand={true}
            />
            <Switch<Routes> render={switch} />
        </BrowserRouter>
    }
}

fn create_routes() -> Vec<BBNavbarLink<Routes>> {
    vec![BBNavbarLinkBuilder::new()
        .to(Routes::Home)
        .label("Home")
        .build()
        .unwrap()]
}
