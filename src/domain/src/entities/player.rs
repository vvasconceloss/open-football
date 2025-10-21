use crate::{
    entities::nation::NationId,
    errors::DomainError,
    value_objects::player::{
        attributes::PlayerAttributes, growth_potential::GrowthPotential, name::PlayerName,
        position::Position, weights::PositionWeights,
    },
};
use chrono::{Datelike, NaiveDate};
use derive_more::{Display, From};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, From, Display)]
pub struct PlayerId(Uuid);

pub struct Player {
    id: PlayerId,
    name: PlayerName,
    nation: NationId,
    position: Position,
    growth_potential: GrowthPotential,
    birth_date: NaiveDate,
    attributes: PlayerAttributes,
}

impl Player {
    pub fn new(
        name: PlayerName,
        nation: NationId,
        position: Position,
        growth_potential: GrowthPotential,
        birth_date: NaiveDate,
        attributes: PlayerAttributes,
    ) -> Result<Self, DomainError> {
        Ok(Self {
            id: PlayerId::from(Uuid::new_v4()),
            name,
            nation,
            position,
            growth_potential,
            birth_date,
            attributes,
        })
    }

    pub fn id(&self) -> &PlayerId {
        &self.id
    }

    pub fn name(&self) -> &PlayerName {
        &self.name
    }

    pub fn nation(&self) -> &NationId {
        &self.nation
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn growth_potential(&self) -> &GrowthPotential {
        &self.growth_potential
    }

    pub fn birth_date(&self) -> &NaiveDate {
        &self.birth_date
    }

    pub fn attributes(&self) -> &PlayerAttributes {
        &self.attributes
    }

    pub fn calculate_market_value(&self, current_date: NaiveDate) -> f32 {
        let base = self.current_ability() * 1_000_000.0;
        let age_factor = 1.0 - ((self.calculate_age(current_date) as f32 - 25.0).abs() / 30.0);

        (base * age_factor).max(0.0)
    }

    pub fn calculate_age(&self, current_date: NaiveDate) -> u8 {
        let mut age = (current_date.year() - self.birth_date.year()) as u8;
        if current_date.month() < self.birth_date.month()
            || (current_date.month() == self.birth_date.month()
                && current_date.day() < self.birth_date.day())
        {
            age = age.saturating_sub(1);
        }
        age
    }

    pub fn current_ability(&self) -> f32 {
        let weights = PositionWeights::for_position(&self.position);
        weights.calculate_ability(&self.attributes)
    }

    pub fn potential_ability(&self) -> f32 {
        let current_ability = self.current_ability();
        current_ability
            + (current_ability * self.growth_potential.as_f32()).min(1.0 - current_ability)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        entities::nation::Nation,
        value_objects::{
            nation::name::NationName,
            player::attributes::{MentalAttributes, PhysicalAttributes, TechnicalAttributes},
            reputation::Reputation,
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
        let nation_reputation = Reputation::new(100);
        let nation_name = NationName::new("Spain".to_string());
        let nation = Nation::new(nation_name.unwrap(), nation_reputation.unwrap()).unwrap();

        let birth_date = NaiveDate::parse_from_str("01-01-2000", "%d-%m-%Y")
            .map_err(|_e| DomainError::Validation("The date of birth is invalid.".to_string()));

        let player = Player::new(
            PlayerName::new("John".to_string(), "Doe".to_string()).unwrap(),
            nation.id().clone(),
            Position::CM,
            GrowthPotential::new(0.25).unwrap(),
            birth_date.unwrap(),
            sample_attributes(),
        )
        .expect("Must create the player");

        assert_eq!(player.name.last_name(), "Doe");
        assert_eq!(player.name.first_name(), "John");

        assert!(player.growth_potential().as_f32() >= 0.0);
        assert!(player.growth_potential().as_f32() <= 1.0);
    }
}
