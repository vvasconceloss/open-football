use derive_more::{Display, From};
use uuid::Uuid;

use crate::errors::DomainError;

#[derive(Debug, PartialEq, Eq, From, Display)]
pub struct NationId(Uuid);

pub struct Nation {
    pub id: NationId,
    pub name: String,
    pub reputation: u8,
}

impl Nation {
    pub fn new(name: String, reputation: u8) -> Result<Self, DomainError> {
        if name.is_empty() {
            return Err(DomainError::Validation(
                "The name of a nation cannot be empty or null.".to_string(),
            ));
        }

        Ok(Self {
            id: NationId::from(Uuid::new_v4()),
            name: name,
            reputation,
        })
    }
}
