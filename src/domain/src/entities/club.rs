use crate::{entities::nation::NationId, errors::DomainError};
use derive_more::{Display, From};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, From, Display)]
pub struct ClubId(Uuid);

pub struct Club {
    pub id: ClubId,
    pub name: String,
    pub reputation: u8,
    pub nation: NationId,
    pub abbreviation: String,
}

impl Club {
    pub fn new(
        name: String,
        reputation: u8,
        nation: NationId,
        abbreviation: String,
    ) -> Result<Self, DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::Validation(
                "The club name cannot be empty or null.".to_string(),
            ));
        }

        Ok(Self {
            id: ClubId::from(Uuid::new_v4()),
            name: name,
            reputation,
            nation,
            abbreviation,
        })
    }
}
