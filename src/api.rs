use crate::{
    database_queries::{
        course_by_id, create_lms_account, list_lms_courses, CourseById, CreateLmsAccount,
        ListLmsCourses,
    },
    errors::LmsError,
    logging::{log_data, log_error},
    stores::courses_store::StoreCourse,
};
use dotenvy_macro::dotenv;
use graphql_client::{GraphQLQuery, Response};

static GRAPHQL_URI: &str = dotenv!("GRAPHQL_URI");

pub async fn get_courses() -> Result<Vec<StoreCourse>, LmsError> {
    let graphql_variables = list_lms_courses::Variables {};
    let body = ListLmsCourses::build_query(graphql_variables);

    Ok(gloo::net::http::Request::post(GRAPHQL_URI)
        .json(&body)
        .map_err(|error| {
            LmsError::FetchingCourses("building request json body".into(), error.to_string())
        })?
        .send()
        .await
        .map_err(|error| LmsError::FetchingCourses("getting response".into(), error.to_string()))?
        .json::<Response<list_lms_courses::ResponseData>>()
        .await
        .map_err(|error| LmsError::FetchingCourses("converting to json".into(), error.to_string()))?
        .data
        .ok_or_else(|| {
            LmsError::FetchingCourses(
                "Extracting data from response".into(),
                "data missing".into(),
            )
        })?
        .lms_courses
        .into_iter()
        .map(|api_course| {
            log_data("api course", &api_course);
            let mut course = StoreCourse::default();
            course.name = api_course.title;
            course.id = api_course.id;
            course.tag = api_course.lms_tag.name.into();
            course.description = api_course.short_description;
            course.price = api_course.price;
            course.long_description = api_course.long_description;
            course.trailer_uri = api_course.trailer_uri;

            course
        })
        .collect::<Vec<StoreCourse>>())
}

pub async fn get_course_by_id(id: i64) -> Result<StoreCourse, LmsError> {
    let graphql_variables = course_by_id::Variables { id };
    let body = CourseById::build_query(graphql_variables);

    let response = gloo::net::http::Request::post(GRAPHQL_URI)
        .json(&body)
        .map_err(|error| {
            LmsError::FetchingCourses("building request json body".into(), error.to_string())
        })?
        .send()
        .await
        .map_err(|error| LmsError::FetchingCourses("getting response".into(), error.to_string()))?
        .json::<Response<course_by_id::ResponseData>>()
        .await
        .map_err(|error| LmsError::FetchingCourses("converting to json".into(), error.to_string()))?
        .data
        .ok_or_else(|| {
            LmsError::FetchingCourses(
                "Extracting data from response".into(),
                "data missing".into(),
            )
        })?;

    if let Some(response_course) = response.lms_courses_by_pk {
        Ok(StoreCourse {
            trailer_uri: response_course.trailer_uri,
            name: response_course.title,
            id: response_course.id,
            description: response_course.short_description,
            tag: response_course.lms_tag.name.into(),
            price: response_course.price,
            long_description: response_course.long_description,
        })
    } else {
        Err(LmsError::CourseNotFound)
    }
}

pub async fn create_account(
    email: String,
    password: String,
) -> Result<create_lms_account::ResponseData, LmsError> {
    let variables = create_lms_account::Variables { email, password };
    let query = CreateLmsAccount::build_query(variables);

    let response = gloo::net::http::Request::post(GRAPHQL_URI)
        .json(&query)
        .map_err(|_error| {
            let my_error =
                LmsError::CreatingAccount("Error creating account".into(), "creating json".into());
            log_error("error sending create account request", &my_error);
            my_error
        })?
        .send()
        .await
        .map_err(|_error| {
            let my_error = LmsError::CreatingAccount(
                "Error creating account".into(),
                "sending request to server".into(),
            );
            log_error("error sending create account request", &my_error);
            my_error
        })?
        .json::<create_lms_account::ResponseData>()
        .await
        .map_err(|_error| {
            let my_error = LmsError::CreatingAccount(
                "Error creating account".into(),
                "error receiving json".into(),
            );
            log_error("error sending create account request", &my_error);
            my_error
        })?;

    Ok(response)
}
