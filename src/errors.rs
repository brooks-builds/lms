use thiserror::Error;

#[derive(Error, Debug, Clone, Copy)]
pub enum LmsError {
    #[error("Error fetching courses from the database")]
    GettingCourses,
}
