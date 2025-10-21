use crate::errors::DomainError;

#[derive(Debug, PartialEq, Eq)]
pub struct CompetitionRules {
    pub points_for_win: u8,
    pub points_for_draw: u8,
    pub promotion_slots: u8,
    pub relegation_slots: u8,
}

impl CompetitionRules {
    pub fn new(
        points_for_win: u8,
        points_for_draw: u8,
        promotion_slots: u8,
        relegation_slots: u8,
    ) -> Result<Self, DomainError> {
        if points_for_win < points_for_draw {
            return Err(DomainError::Validation(
                "Points for win must be greater than or equal to points for draw.".to_string(),
            ));
        }

        Ok(Self {
            points_for_win,
            points_for_draw,
            promotion_slots,
            relegation_slots,
        })
    }

    pub fn points_for_win(&self) -> u8 {
        self.points_for_win
    }

    pub fn points_for_draw(&self) -> u8 {
        self.points_for_draw
    }

    pub fn promotion_slots(&self) -> u8 {
        self.promotion_slots
    }

    pub fn relegation_slots(&self) -> u8 {
        self.relegation_slots
    }
}
