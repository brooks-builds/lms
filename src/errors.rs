use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum LmsError {
    #[error("{0}: {1}")]
    FetchingCourses(String, String),
    #[error("console.log exists")]
    LeftInLog,
    #[error("could not find course")]
    CourseNotFound,
    #[error("error saving cookie")]
    SavingCookie,
    #[error("{0}: {1}")]
    CreatingAccount(String, String),
    #[error("Error sending request to graphQL: {0}")]
    SendingToGraphqlApi(String),
}
