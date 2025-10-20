use crate::{
    entities::{club::ClubId, player::PlayerId},
    errors::DomainError,
    value_objects::money::Money,
};
use chrono::NaiveDate;
use derive_more::{Display, From};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, From, Display)]
pub struct ContractId(Uuid);

pub struct Contract {
    pub id: ContractId,
    pub club_id: ClubId,
    pub player_id: PlayerId,
    pub end_date: NaiveDate,
    pub start_date: NaiveDate,
    pub weekly_wage: Money,
}

impl Contract {
    pub fn new(
        club_id: ClubId,
        player_id: PlayerId,
        end_date: NaiveDate,
        start_date: NaiveDate,
        weekly_wage: Money,
    ) -> Result<Self, DomainError> {
        Ok(Contract {
            id: ContractId::from(Uuid::new_v4()),
            club_id,
            player_id,
            end_date,
            start_date,
            weekly_wage,
        })
    }
}
