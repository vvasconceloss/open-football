#[derive(Debug, Clone)]
pub struct MentalAttributes {
    pub vision: u8,
    pub composure: u8,
    pub positioning: u8,
}

#[derive(Debug, Clone)]
pub struct PhysicalAttributes {
    pub pace: u8,
    pub stamina: u8,
    pub strength: u8,
}

#[derive(Debug, Clone)]
pub struct TechnicalAttributes {
    pub passing: u8,
    pub heading: u8,
    pub tackling: u8,
    pub dribbling: u8,
    pub finishing: u8,
}

#[derive(Debug, Clone)]
pub struct GoalkeepingAttributes {
    pub kicking: u8,
    pub handling: u8,
    pub reflexes: u8,
    pub positioning: u8,
    pub aerial_reach: u8,
}

#[derive(Debug, Clone)]
pub enum PlayerAttributes {
    Field {
        mental: MentalAttributes,
        physical: PhysicalAttributes,
        technical: TechnicalAttributes,
    },
    Goalkeeper {
        mental: MentalAttributes,
        physical: PhysicalAttributes,
        technical: GoalkeepingAttributes,
    },
}
