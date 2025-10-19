use crate::value_objects::player::position::Position;
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
}
