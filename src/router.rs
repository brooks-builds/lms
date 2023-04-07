use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
    articles::Articles, auth_redirect::AuthRedirect, course_details::CourseDetails,
    courses::Courses, create_account::CreateAccount, create_course::CreateCourse, home::Home,
    login::Login, tags::Tags,
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
    #[at("/create_course")]
    CreateCourse,
    #[at("/tags")]
    Tags,
    #[at("/articles")]
    Articles,
}

pub fn switch(routes: Routes) -> Html {
    match routes {
        Routes::Home => html! { <Home /> },
        Routes::CreateAccount => html! { <CreateAccount /> },
        Routes::Login => html! { <Login /> },
        Routes::Courses => html! { <Courses /> },
        Routes::CourseDetails { id } => html! { <CourseDetails {id} /> },
        Routes::AuthRedirect => html! { <AuthRedirect />},
        Routes::CreateCourse => html! { <CreateCourse />},
        Routes::Tags => html! { <Tags />},
        Routes::Articles => html! { <Articles />},
    }
}
