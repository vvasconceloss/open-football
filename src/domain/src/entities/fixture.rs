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

pub enum FixtureStatus {
    Played,
    Scheduled,
    Postponed,
}

pub struct Fixture {
    pub id: FixtureId,
    pub date: NaiveDate,
    pub home_club_id: ClubId,
    pub away_club_id: ClubId,
    pub status: FixtureStatus,
    pub result: Option<FixtureResult>,
    pub competition_id: CompetitionId,
}

impl Fixture {
    pub fn new(
        date: NaiveDate,
        home_club_id: ClubId,
        away_club_id: ClubId,
        result: Option<FixtureResult>,
        competition_id: CompetitionId,
    ) -> Result<Self, DomainError> {
        Ok(Self {
            id: FixtureId::from(Uuid::new_v4()),
            date,
            home_club_id,
            away_club_id,
            status: FixtureStatus::Scheduled,
            result,
            competition_id,
        })
    }
}
