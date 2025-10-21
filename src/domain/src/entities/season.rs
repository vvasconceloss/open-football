use crate::{entities::competition::Competition, errors::DomainError};
use chrono::{Datelike, NaiveDate};
use derive_more::{Display, From};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, From, Display)]
pub struct SeasonId(Uuid);

pub struct Season {
    pub id: SeasonId,
    pub end_year: u32,
    pub start_year: u32,
    pub end_date: NaiveDate,
    pub start_date: NaiveDate,
    pub competitions: Vec<Competition>,
}

impl Season {
    pub fn new(
        start_year: u32,
        end_year: u32,
        start_date: NaiveDate,
        end_date: NaiveDate,
        competitions: Vec<Competition>,
    ) -> Result<Self, DomainError> {
        if start_date >= end_date {
            return Err(DomainError::Validation(
                "Season start date must be strictly before the end date.".to_string(),
            ));
        }

        if start_year > end_year {
            return Err(DomainError::Validation(
                "Season start year cannot be greater than the end year.".to_string(),
            ));
        }

        if start_date.year() as u32 != start_year {
            return Err(DomainError::Validation(
                "Start date's year does not match start year.".to_string(),
            ));
        }
        if end_date.year() as u32 != end_year {
            return Err(DomainError::Validation(
                "End date's year does not match end year.".to_string(),
            ));
        }

        Ok(Self {
            id: SeasonId::from(Uuid::new_v4()),
            start_year,
            end_year,
            start_date,
            end_date,
            competitions,
        })
    }

    pub fn id(&self) -> &SeasonId {
        &self.id
    }

    pub fn start_year(&self) -> u32 {
        self.start_year
    }

    pub fn end_year(&self) -> u32 {
        self.end_year
    }

    pub fn start_date(&self) -> NaiveDate {
        self.start_date
    }

    pub fn end_date(&self) -> NaiveDate {
        self.end_date
    }

    pub fn competitions(&self) -> &[Competition] {
        &self.competitions
    }

    pub fn add_competition(&mut self, competition: Competition) {
        self.competitions.push(competition);
    }

    pub fn remove_competition(&mut self, competition_id: &super::competition::CompetitionId) {
        self.competitions.retain(|comp| comp.id() != competition_id);
    }
}
