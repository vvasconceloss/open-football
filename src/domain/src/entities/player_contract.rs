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
    id: ContractId,
    club_id: ClubId,
    player_id: PlayerId,
    end_date: NaiveDate,
    start_date: NaiveDate,
    weekly_wage: Money,
}

impl Contract {
    pub fn new(
        club_id: ClubId,
        player_id: PlayerId,
        end_date: NaiveDate,
        start_date: NaiveDate,
        weekly_wage: Money,
    ) -> Result<Self, DomainError> {
        if start_date >= end_date {
            return Err(DomainError::Validation(
                "Contract start date must be strictly before the end date.".to_string(),
            ));
        }

        Ok(Contract {
            id: ContractId::from(Uuid::new_v4()),
            club_id,
            player_id,
            end_date,
            start_date,
            weekly_wage,
        })
    }

    pub fn id(&self) -> &ContractId {
        &self.id
    }

    pub fn club_id(&self) -> &ClubId {
        &self.club_id
    }

    pub fn player_id(&self) -> &PlayerId {
        &self.player_id
    }

    pub fn end_date(&self) -> &NaiveDate {
        &self.end_date
    }

    pub fn start_date(&self) -> &NaiveDate {
        &self.start_date
    }

    pub fn weekly_wage(&self) -> &Money {
        &self.weekly_wage
    }

    pub fn extend_end_date(&mut self, new_end_date: NaiveDate) -> Result<(), DomainError> {
        if new_end_date <= self.end_date {
            return Err(DomainError::Unknown(
                "New end date must be after the current contract end date.".to_string(),
            ));
        }

        self.end_date = new_end_date;
        Ok(())
    }

    pub fn update_wage(&mut self, new_wage: Money) {
        self.weekly_wage = new_wage;
    }
}
