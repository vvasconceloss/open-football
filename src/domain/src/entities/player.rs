use crate::{
    errors::DomainError,
    value_objects::player::{
        attributes::PlayerAttributes, position::Position, weights::PositionWeights,
    },
};
use derive_more::{Display, From};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, From, Display)]
pub struct PlayerId(Uuid);

pub struct Player {
    pub id: PlayerId,
    pub last_name: String,
    pub first_name: String,
    pub position: Position,
    pub attributes: PlayerAttributes,
}

impl Player {
    pub fn new(
        last_name: String,
        first_name: String,
        position: Position,
        attributes: PlayerAttributes,
    ) -> Result<Self, DomainError> {
        Ok(Self {
            id: PlayerId::from(Uuid::new_v4()),
            last_name: last_name,
            first_name: first_name,
            position,
            attributes,
        })
    }

    pub fn current_ability(&self) -> f32 {
        let weights = PositionWeights::for_position(&self.position);
        weights.calculate_ability(&self.attributes)
    }
}
