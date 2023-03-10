use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum LmsError {
    #[error("console.log exists")]
    LeftInLog,
    #[error("could not find course")]
    CourseNotFound,
    #[error("error saving cookie")]
    SavingCookie,
    #[error("error getting cookie: {0}")]
    GettingCookie(String),
    #[error("Error sending request to graphQL: {0}")]
    SendingToGraphqlApi(String),
    #[error("Error building alert store: {0}")]
    BuildingAlertStore(String),
    #[error("Error handling auth redirect: {0}")]
    HandleAuthRedirectError(String),
}
