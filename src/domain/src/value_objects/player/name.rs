use crate::errors::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerName {
    last_name: String,
    first_name: String,
}

impl PlayerName {
    pub fn new(first: String, last: String) -> Result<Self, DomainError> {
        let trimmed_first = first.trim().to_string();
        let trimmed_last = last.trim().to_string();

        if trimmed_first.is_empty() || trimmed_last.is_empty() {
            return Err(DomainError::Validation(
                "Both first name and last name must be provided and cannot be empty.".to_string(),
            ));
        }

        Ok(PlayerName {
            first_name: trimmed_first,
            last_name: trimmed_last,
        })
    }

    pub fn first_name(&self) -> &str {
        &self.first_name
    }

    pub fn last_name(&self) -> &str {
        &self.last_name
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}
