use crate::errors::DomainError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reputation(u8);

impl Reputation {
    pub fn new(reputation: u8) -> Result<Self, DomainError> {
        if !(1..=100).contains(&reputation) {
            return Err(DomainError::Validation(
                "Reputation must be between 1 and 100.".to_string(),
            ));
        }

        Ok(Reputation(reputation))
    }

    pub fn as_u8(&self) -> u8 {
        self.0
    }
}
