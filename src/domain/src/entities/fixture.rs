use crate::{
    entities::{club::ClubId, competition::CompetitionId},
    errors::DomainError,
    value_objects::fixture::fixture_result::FixtureResult,
};
use chrono::NaiveDate;
use derive_more::{Display, From};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, From, Display)]
pub struct FixtureId(Uuid);

#[derive(Debug, PartialEq, Eq)]
pub enum FixtureStatus {
    Played,
    Scheduled,
    Postponed,
}

#[derive(Debug, PartialEq)]
pub struct Fixture {
    id: FixtureId,
    date: NaiveDate,
    home_club_id: ClubId,
    away_club_id: ClubId,
    status: FixtureStatus,
    result: Option<FixtureResult>,
    competition_id: CompetitionId,
}

impl Fixture {
    pub fn new(
        date: NaiveDate,
        home_club_id: ClubId,
        away_club_id: ClubId,
        competition_id: CompetitionId,
    ) -> Result<Self, DomainError> {
        if home_club_id == away_club_id {
            return Err(DomainError::Validation(
                "Home club and away club cannot be the same.".to_string(),
            ));
        }

        Ok(Self {
            id: FixtureId::from(Uuid::new_v4()),
            date,
            home_club_id,
            away_club_id,
            status: FixtureStatus::Scheduled,
            result: None,
            competition_id,
        })
    }

    pub fn id(&self) -> &FixtureId {
        &self.id
    }

    pub fn date(&self) -> &NaiveDate {
        &self.date
    }

    pub fn home_club_id(&self) -> &ClubId {
        &self.home_club_id
    }

    pub fn away_club_id(&self) -> &ClubId {
        &self.away_club_id
    }

    pub fn status(&self) -> &FixtureStatus {
        &self.status
    }

    pub fn result(&self) -> &Option<FixtureResult> {
        &self.result
    }

    pub fn competition_id(&self) -> &CompetitionId {
        &self.competition_id
    }

    pub fn record_result(&mut self, final_result: FixtureResult) -> Result<(), DomainError> {
        if self.status == FixtureStatus::Played {
            return Err(DomainError::Unknown(
                "Cannot change the result of a played fixture.".to_string(),
            ));
        }

        self.result = Some(final_result);
        self.status = FixtureStatus::Played;
        Ok(())
    }

    pub fn postpone(&mut self) -> Result<(), DomainError> {
        if self.status == FixtureStatus::Played {
            return Err(DomainError::Unknown(
                "Cannot postpone a game that has already been played.".to_string(),
            ));
        }
        self.status = FixtureStatus::Postponed;
        Ok(())
    }
}
