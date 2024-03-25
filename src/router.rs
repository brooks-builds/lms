use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
    auth_redirect::AuthRedirect, course_access::CourseAccess,
    course_access_article::CourseAccessArticle, course_articles::CourseArticles,
    course_details::CourseDetails, course_purchase::CoursePurchase, courses::Courses,
    create_article::CreateArticle, create_course::CreateCourse, home::Home, tags::Tags,
};

#[derive(Clone, Routable, PartialEq, Debug, Default)]
pub enum Routes {
    #[default]
    #[at("/")]
    Home,
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
    #[at("/create_article")]
    CreateArticle,
    #[at("/course_articles/:id")]
    CourseArticles { id: i64 },
    #[at("/courses/:id/access")]
    CourseAccess { id: i64 },
    #[at("/courses/:course_id/access/:article_id")]
    CourseAccessArticle { course_id: i64, article_id: i64 },
    #[at("/courses/:course_id/purchase")]
    CoursePurchase { course_id: i64 },
}

pub fn switch(routes: Routes) -> Html {
    match routes {
        Routes::Home => html! { <Home /> },
        Routes::Courses => html! { <Courses /> },
        Routes::CourseDetails { id } => html! { <CourseDetails {id} /> },
        Routes::AuthRedirect => html! { <AuthRedirect />},
        Routes::CreateCourse => html! { <CreateCourse />},
        Routes::Tags => html! { <Tags />},
        Routes::CreateArticle => html! { <CreateArticle />},
        Routes::CourseArticles { id } => html! { <CourseArticles course_id={id}/>},
        Routes::CourseAccess { id } => html! { <CourseAccess {id}/>},
        Routes::CourseAccessArticle {
            course_id,
            article_id,
        } => html! { <CourseAccessArticle {course_id} {article_id} />},
        Routes::CoursePurchase { course_id } => html! { <CoursePurchase {course_id} />},
    }
}
