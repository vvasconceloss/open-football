use crate::errors::DomainError;
use derive_more::{Display, From};

#[derive(Debug, Clone, Copy, PartialEq, From, Display)]
pub struct GrowthPotential(f32);

impl GrowthPotential {
    pub fn new(potential: f32) -> Result<Self, DomainError> {
        if !(0.0..=1.0).contains(&potential) {
            return Err(DomainError::Validation(
                "Growth potential must be between 0.0 and 1.0.".to_string(),
            ));
        }

        Ok(GrowthPotential(potential))
    }

    pub fn as_f32(&self) -> f32 {
        self.0
    }
}
