use crate::{
    entities::{club::ClubId, fixture::FixtureId},
    value_objects::competitions::standing::Standing,
};
use derive_more::{Display, From};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, From, Display)]
pub struct CompetitionId(Uuid);

pub enum CompetitionType {
    League,
    Knockout,
    GroupStage,
    Friendly,
    Custom,
}

pub struct CompetitionRules {
    pub points_for_win: u8,
    pub points_for_draw: u8,
    pub promotion_slots: u8,
    pub relegation_slots: u8,
}

pub struct Competition {
    pub id: CompetitionId,
    pub name: String,
    pub rules: CompetitionRules,
    pub format: CompetitionType,
    pub fixtures: Vec<FixtureId>,
    pub participants: Vec<ClubId>,
    pub standings: Option<Standing>,
}
