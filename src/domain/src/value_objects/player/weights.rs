use crate::value_objects::player::{attributes::PlayerAttributes, position::Position};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Attribute {
    Vision,
    Composure,
    Positioning,
    Pace,
    Stamina,
    Strength,
    Passing,
    Heading,
    Tackling,
    Dribbling,
    Finishing,
    Kicking,
    Handling,
    Reflexes,
    AerialReach,
}

#[derive(Debug, Clone)]
pub struct PositionWeights {
    pub position: Position,
    pub weights: HashMap<Attribute, f32>,
}

impl PositionWeights {
    pub fn total_weights(&self) -> f32 {
        self.weights.values().sum()
    }

    pub fn is_valid(&self) -> bool {
        (self.total_weights() - 1.0).abs() < f32::EPSILON
    }

    pub fn for_position(position: &Position) -> Self {
        use Attribute::*;
        let mut weights = HashMap::new();

        let attributes = [
            Vision,
            Composure,
            Positioning,
            Pace,
            Stamina,
            Strength,
            Passing,
            Heading,
            Tackling,
            Dribbling,
            Finishing,
            Kicking,
            Handling,
            Reflexes,
            AerialReach,
        ];

        for attribute in attributes {
            weights.insert(attribute, 0.0);
        }

        match position {
            Position::GK => {
                weights.insert(Kicking, 0.15);
                weights.insert(Reflexes, 0.25);
                weights.insert(Handling, 0.20);
                weights.insert(Strength, 0.05);
                weights.insert(Composure, 0.10);
                weights.insert(AerialReach, 0.10);
                weights.insert(Positioning, 0.15);
            }
            Position::CB => {
                weights.insert(Tackling, 0.20);
                weights.insert(Strength, 0.15);
                weights.insert(Positioning, 0.15);
                weights.insert(Heading, 0.10);
                weights.insert(Passing, 0.10);
                weights.insert(Composure, 0.10);
                weights.insert(Pace, 0.10);
                weights.insert(Stamina, 0.05);
                weights.insert(Vision, 0.05);
            }
            Position::RB | Position::LB => {
                weights.insert(Pace, 0.15);
                weights.insert(Stamina, 0.15);
                weights.insert(Tackling, 0.15);
                weights.insert(Positioning, 0.10);
                weights.insert(Passing, 0.10);
                weights.insert(Dribbling, 0.10);
                weights.insert(Composure, 0.10);
                weights.insert(Vision, 0.05);
                weights.insert(Strength, 0.05);
                weights.insert(Finishing, 0.05);
            }
            Position::RWB | Position::LWB => {
                weights.insert(Pace, 0.15);
                weights.insert(Stamina, 0.15);
                weights.insert(Passing, 0.10);
                weights.insert(Tackling, 0.10);
                weights.insert(Dribbling, 0.10);
                weights.insert(Positioning, 0.10);
                weights.insert(Composure, 0.10);
                weights.insert(Vision, 0.10);
                weights.insert(Strength, 0.05);
                weights.insert(Finishing, 0.05);
            }
            Position::DM => {
                weights.insert(Tackling, 0.15);
                weights.insert(Positioning, 0.15);
                weights.insert(Passing, 0.15);
                weights.insert(Strength, 0.10);
                weights.insert(Composure, 0.10);
                weights.insert(Stamina, 0.10);
                weights.insert(Vision, 0.10);
                weights.insert(Pace, 0.05);
                weights.insert(Dribbling, 0.05);
                weights.insert(Heading, 0.05);
            }
            Position::CM => {
                weights.insert(Passing, 0.15);
                weights.insert(Vision, 0.15);
                weights.insert(Positioning, 0.10);
                weights.insert(Stamina, 0.10);
                weights.insert(Composure, 0.10);
                weights.insert(Dribbling, 0.10);
                weights.insert(Strength, 0.10);
                weights.insert(Pace, 0.10);
                weights.insert(Finishing, 0.05);
                weights.insert(Heading, 0.05);
            }
            Position::AM => {
                weights.insert(Vision, 0.15);
                weights.insert(Passing, 0.15);
                weights.insert(Dribbling, 0.15);
                weights.insert(Finishing, 0.15);
                weights.insert(Composure, 0.10);
                weights.insert(Positioning, 0.10);
                weights.insert(Pace, 0.10);
                weights.insert(Stamina, 0.05);
                weights.insert(Strength, 0.05);
            }
            Position::RM | Position::LM => {
                weights.insert(Pace, 0.15);
                weights.insert(Dribbling, 0.15);
                weights.insert(Passing, 0.10);
                weights.insert(Vision, 0.10);
                weights.insert(Finishing, 0.10);
                weights.insert(Stamina, 0.10);
                weights.insert(Positioning, 0.10);
                weights.insert(Composure, 0.10);
                weights.insert(Strength, 0.05);
                weights.insert(Heading, 0.05);
            }
            Position::RW | Position::LW => {
                weights.insert(Pace, 0.20);
                weights.insert(Dribbling, 0.15);
                weights.insert(Finishing, 0.15);
                weights.insert(Vision, 0.10);
                weights.insert(Passing, 0.10);
                weights.insert(Positioning, 0.10);
                weights.insert(Composure, 0.10);
                weights.insert(Stamina, 0.05);
                weights.insert(Strength, 0.05);
            }
            Position::SS | Position::ST => {
                weights.insert(Finishing, 0.25);
                weights.insert(Positioning, 0.15);
                weights.insert(Heading, 0.15);
                weights.insert(Strength, 0.10);
                weights.insert(Composure, 0.10);
                weights.insert(Pace, 0.10);
                weights.insert(Dribbling, 0.05);
                weights.insert(Passing, 0.05);
                weights.insert(Vision, 0.05);
            }
        }

        Self {
            weights,
            position: position.clone(),
        }
    }

    pub fn calculate_ability(&self, attributes: &PlayerAttributes) -> f32 {
        let mut total = 0.0;

        for (attribute, weight) in &self.weights {
            if let Some(value) = attributes.get_attribute_value(attribute) {
                total += (value as f32 / 100.0) * weight;
            }
        }

        total
    }
}
