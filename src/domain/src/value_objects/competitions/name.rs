use crate::errors::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompetitionName(String);

impl CompetitionName {
    pub fn new(name: String) -> Result<Self, DomainError> {
        let trimmed_name = name.trim().to_string();

        if trimmed_name.is_empty() {
            return Err(DomainError::Validation(
                "The competition name cannot be empty or null.".to_string(),
            ));
        }

        if trimmed_name.len() > 100 {
            return Err(DomainError::Validation(
                "The competition name is too long.".to_string(),
            ));
        }

        Ok(CompetitionName(trimmed_name))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
