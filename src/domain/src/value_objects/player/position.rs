#[derive(Debug, Clone)]
pub enum Position {
    GK,  // Goalkeeper
    RB,  // Right Back
    CB,  // Center Back
    LB,  // Left Back
    RWB, // Right Wing-back
    LWB, // Left Wing-back
    DM,  // Defensive Midfielder
    CM,  // Center Midfielder
    AM,  // Attack Midfielder
    LM,  // Left Midfielder
    RM,  // Right Midfielder
    LW,  // Left Winger
    RW,  // Right Winger
    SS,  // Second Striker
    ST,  // Striker
}

impl Position {
    pub fn category(&self) -> &'static str {
        match self {
            Position::GK => "Goalkeeer",
            Position::RB | Position::CB | Position::LB | Position::RWB | Position::LWB => {
                "Defensive"
            }
            Position::DM | Position::CM | Position::AM | Position::LM | Position::RM => "Midfield",
            Position::LW | Position::RW | Position::SS | Position::ST => "Attacking",
        }
    }
}
