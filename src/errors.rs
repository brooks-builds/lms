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
    #[error("There was an error, please refresh your browser and try again")]
    AuthGetState,
    #[error("Auth0 state does not match the saved state")]
    AuthStateDoesNotMatch,
    #[error("Auth0 state was not found in the cookie")]
    AuthStateMissing,
}
