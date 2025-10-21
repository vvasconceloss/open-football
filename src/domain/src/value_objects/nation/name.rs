use crate::errors::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NationName(String);

impl NationName {
    pub fn new(name: String) -> Result<Self, DomainError> {
        let trimmed_name = name.trim().to_string();

        if trimmed_name.is_empty() {
            return Err(DomainError::Validation(
                "The name of a nation cannot be empty or null.".to_string(),
            ));
        }

        Ok(NationName(trimmed_name))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
