use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Domain violation: {0}")]
    DomainViolation(String),

    #[error("Entity not found: {0}")]
    EntityNotFound(String),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Conflict: {0}")]
    ConflictError(String),

    #[error("Persistence failure: {0}")]
    PersistenceError(String),

    #[error("Connection failure: {0}")]
    ConnectionError(String),

    #[error("Serialization failure: {0}")]
    SerializationError(String),

    #[error("External service failure: {0}")]
    ExternalServiceError(String),

    #[error("Timeout: {0}")]
    TimeoutError(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}
