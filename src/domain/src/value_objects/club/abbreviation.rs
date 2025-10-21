use crate::errors::DomainError;

#[derive(Debug, PartialEq, Eq)]
pub struct ClubAbbreviation(String);

impl ClubAbbreviation {
    pub fn new(abbreviation: String) -> Result<Self, DomainError> {
        if abbreviation.trim().len() != 3 {
            return Err(DomainError::Validation(
                "Abbreviation must be exactly 3 characters.".to_string(),
            ));
        }

        Ok(ClubAbbreviation(abbreviation))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
