use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("The entity '{0}' was not found.")]
    NotFound(String),

    #[error("A validation error has occurred.")]
    Validation(String),

    #[error("An unexpected error has occurred.")]
    Unknown(String),
}
