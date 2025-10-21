use crate::{
    entities::{nation::NationId, player_contract::ContractId},
    errors::DomainError,
    value_objects::{
        club::{abbreviation::ClubAbbreviation, name::ClubName},
        reputation::Reputation,
    },
};
use derive_more::{Display, From};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, From, Display)]
pub struct ClubId(Uuid);

pub struct Club {
    id: ClubId,
    name: ClubName,
    nation: NationId,
    squad: Vec<ContractId>,
    reputation: Reputation,
    abbreviation: ClubAbbreviation,
}

impl Club {
    pub fn new(
        name: ClubName,
        nation: NationId,
        squad: Vec<ContractId>,
        reputation: Reputation,
        abbreviation: ClubAbbreviation,
    ) -> Result<Self, DomainError> {
        Ok(Self {
            id: ClubId::from(Uuid::new_v4()),
            name,
            nation,
            squad,
            reputation,
            abbreviation,
        })
    }

    pub fn id(&self) -> &ClubId {
        &self.id
    }

    pub fn name(&self) -> &ClubName {
        &self.name
    }

    pub fn nation(&self) -> &NationId {
        &self.nation
    }

    pub fn squad(&self) -> &[ContractId] {
        &self.squad
    }

    pub fn reputation(&self) -> &Reputation {
        &self.reputation
    }

    pub fn abbreviation(&self) -> &ClubAbbreviation {
        &self.abbreviation
    }

    pub fn change_reputation(&mut self, new_reputation: Reputation) {
        self.reputation = new_reputation
    }

    pub fn add_player(&mut self, contract: ContractId) {
        if !self.squad.contains(&contract) {
            self.squad.push(contract)
        }
    }

    pub fn remove_player(&mut self, contract_remove: &ContractId) {
        self.squad.retain(|contract| contract != contract_remove);
    }
}
