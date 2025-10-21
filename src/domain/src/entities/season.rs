use crate::entities::competition::Competition;
use chrono::NaiveDate;
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
