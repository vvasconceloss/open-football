use crate::{
    entities::nation::NationId,
    errors::DomainError,
    value_objects::player::{
        attributes::PlayerAttributes, position::Position, weights::PositionWeights,
    },
};
use chrono::{Datelike, NaiveDate};
use derive_more::{Display, From};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, From, Display)]
pub struct PlayerId(Uuid);

pub struct Player {
    pub id: PlayerId,
    pub nation: NationId,
    pub last_name: String,
    pub first_name: String,
    pub position: Position,
    pub growth_potential: f32,
    pub birth_date: NaiveDate,
    pub attributes: PlayerAttributes,
}

impl Player {
    pub fn new(
        nation: NationId,
        last_name: String,
        first_name: String,
        position: Position,
        growth_potential: f32,
        birth_date: NaiveDate,
        attributes: PlayerAttributes,
    ) -> Result<Self, DomainError> {
        if last_name.trim().is_empty() || first_name.trim().is_empty() {
            return Err(DomainError::Validation(
                "The player's name cannot be empty or null.".to_string(),
            ));
        }

        if !(0.0..=1.0).contains(&growth_potential) {
            return Err(DomainError::Validation(
                "Growth potential must be between 0.0 and 1.0".to_string(),
            ));
        }

        Ok(Self {
            id: PlayerId::from(Uuid::new_v4()),
            nation,
            last_name: last_name,
            first_name: first_name,
            position,
            growth_potential,
            birth_date,
            attributes,
        })
    }

    pub fn calculate_age(&self, current_date: NaiveDate) -> u8 {
        (current_date.year() - self.birth_date.year()) as u8
    }

    pub fn current_ability(&self) -> f32 {
        let weights = PositionWeights::for_position(&self.position);
        weights.calculate_ability(&self.attributes)
    }

    pub fn potential_ability(&self) -> f32 {
        let current_ability = self.current_ability();
        current_ability + (current_ability * self.growth_potential).min(1.0 - current_ability)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        entities::nation::Nation,
        value_objects::player::attributes::{
            MentalAttributes, PhysicalAttributes, TechnicalAttributes,
        },
    };

    use super::*;

    fn sample_attributes() -> PlayerAttributes {
        PlayerAttributes::Field {
            mental: MentalAttributes {
                vision: 70,
                composure: 65,
                positioning: 60,
            },
            physical: PhysicalAttributes {
                pace: 75,
                stamina: 70,
                strength: 68,
            },
            technical: TechnicalAttributes {
                passing: 72,
                heading: 50,
                tackling: 65,
                dribbling: 74,
                finishing: 60,
            },
        }
    }

    #[test]
    fn create_player_successfully() {
        let nation = Nation::new("Spain".to_string(), 100);
        let birth_date = NaiveDate::parse_from_str(&"01-01-2000", "%d-%m-%Y")
            .map_err(|_e| DomainError::Validation("The date of birth is invalid.".to_string()));

        let player = Player::new(
            nation.unwrap().id,
            "Doe".to_string(),
            "John".to_string(),
            Position::CM,
            0.25,
            birth_date.unwrap(),
            sample_attributes(),
        )
        .expect("Must create the player");

        assert_eq!(player.last_name, "Doe");
        assert_eq!(player.first_name, "John");
        assert!(player.growth_potential >= 0.0);
        assert!(player.growth_potential <= 1.0);
    }
}
