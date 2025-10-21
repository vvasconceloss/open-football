use crate::{
    entities::{club::ClubId, fixture::FixtureId},
    errors::DomainError,
    value_objects::competitions::{
        name::CompetitionName, rules::CompetitionRules, standing::Standing,
    },
};
use derive_more::{Display, From};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, From, Display)]
pub struct CompetitionId(Uuid);

#[derive(Debug, PartialEq, Eq, From, Display)]
pub enum CompetitionType {
    League,
    Knockout,
    GroupStage,
    Friendly,
    Custom,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Competition {
    id: CompetitionId,
    name: CompetitionName,
    rules: CompetitionRules,
    format: CompetitionType,
    fixtures: Vec<FixtureId>,
    participants: Vec<ClubId>,
    standings: Option<Standing>,
}

impl Competition {
    pub fn new(
        name: CompetitionName,
        rules: CompetitionRules,
        format: CompetitionType,
        fixtures: Vec<FixtureId>,
        participants: Vec<ClubId>,
        standings: Option<Standing>,
    ) -> Result<Self, DomainError> {
        if participants.len() < 2 && format != CompetitionType::Friendly {
            return Err(DomainError::Validation(
                "Non-friendly competition must have at least two participants.".to_string(),
            ));
        }

        Ok(Self {
            id: CompetitionId::from(Uuid::new_v4()),
            name,
            rules,
            format,
            fixtures,
            participants,
            standings,
        })
    }

    pub fn id(&self) -> &CompetitionId {
        &self.id
    }

    pub fn name(&self) -> &CompetitionName {
        &self.name
    }

    pub fn rules(&self) -> &CompetitionRules {
        &self.rules
    }

    pub fn format(&self) -> &CompetitionType {
        &self.format
    }

    pub fn fixtures(&self) -> &[FixtureId] {
        &self.fixtures
    }

    pub fn participants(&self) -> &[ClubId] {
        &self.participants
    }

    pub fn standings(&self) -> &Option<Standing> {
        &self.standings
    }

    pub fn add_participant(&mut self, club_id: ClubId) {
        if !self.participants.contains(&club_id) {
            self.participants.push(club_id);
        }
    }

    pub fn remove_participant(&mut self, club_id: &ClubId) {
        self.participants
            .retain(|participant| participant != club_id);
    }

    pub fn update_standings(&mut self, new_standings: Standing) {
        self.standings = Some(new_standings);
    }
}
