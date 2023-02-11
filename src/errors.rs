use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum LmsError {
    #[error("{0}: {1}")]
    FetchingCourses(String, String),
    #[error("console.log exists")]
    LeftInLog,
}
