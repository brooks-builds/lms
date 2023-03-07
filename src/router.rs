use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
    course_details::CourseDetails, courses::Courses, create_account::CreateAccount, home::Home,
    login::Login, auth_redirect::AuthRedirect,
};

#[derive(Clone, Routable, PartialEq)]
pub enum Routes {
    #[at("/")]
    Home,
    #[at("/auth/create_account")]
    CreateAccount,
    #[at("/auth/login")]
    Login,
    #[at("/courses")]
    Courses,
    #[at("/courses/:id")]
    CourseDetails { id: i64 },
    #[at("/auth/redirect")]
    AuthRedirect,
}

pub fn switch(routes: Routes) -> Html {
    match routes {
        Routes::Home => html! { <Home /> },
        Routes::CreateAccount => html! { <CreateAccount /> },
        Routes::Login => html! { <Login /> },
        Routes::Courses => html! { <Courses /> },
        Routes::CourseDetails { id } => html! { <CourseDetails {id} /> },
        Routes::AuthRedirect => html! { <AuthRedirect />}
    }
}
