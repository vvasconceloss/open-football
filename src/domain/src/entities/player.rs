use crate::errors::DomainError;
use derive_more::{Display, From};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, From, Display)]
pub struct PlayerId(Uuid);

pub struct Player {
    pub id: PlayerId,
    pub last_name: String,
    pub first_name: String,
}

impl Player {
    pub fn new(last_name: String, first_name: String) -> Result<Self, DomainError> {
        Ok(Self {
            id: PlayerId::from(Uuid::new_v4()),
            last_name: last_name,
            first_name: first_name,
        })
    }
}
