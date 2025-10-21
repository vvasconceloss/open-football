use crate::{
    errors::DomainError,
    value_objects::{nation::name::NationName, reputation::Reputation},
};
use derive_more::{Display, From};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, From, Display, Clone)]
pub struct NationId(Uuid);

pub struct Nation {
    id: NationId,
    name: NationName,
    reputation: Reputation,
}

impl Nation {
    pub fn new(name: NationName, reputation: Reputation) -> Result<Self, DomainError> {
        Ok(Self {
            id: NationId::from(Uuid::new_v4()),
            name,
            reputation,
        })
    }

    pub fn id(&self) -> &NationId {
        &self.id
    }

    pub fn name(&self) -> &NationName {
        &self.name
    }

    pub fn reputation(&self) -> &Reputation {
        &self.reputation
    }

    pub fn increase_reputation(&mut self, new_reputation: Reputation) {
        if new_reputation.as_u8() > self.reputation.as_u8() {
            self.reputation = new_reputation;
        }
    }
}
