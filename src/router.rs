use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
    course_profile::CourseProfile, courses::Courses, create_account::CreateAccount, home::Home,
    login::Login,
};

#[derive(Clone, Routable, PartialEq)]
pub enum Routes {
    #[at("/")]
    Home,
    #[at("/create_account")]
    CreateAccount,
    #[at("/login")]
    Login,
    #[at("/courses")]
    Courses,
    #[at("/courses/:id")]
    CourseProfile { id: i64 },
}

pub fn switch(routes: Routes) -> Html {
    match routes {
        Routes::Home => html! { <Home /> },
        Routes::CreateAccount => html! { <CreateAccount /> },
        Routes::Login => html! { <Login /> },
        Routes::Courses => html! { <Courses /> },
        Routes::CourseProfile { id } => html! { <CourseProfile {id} /> },
    }
}
