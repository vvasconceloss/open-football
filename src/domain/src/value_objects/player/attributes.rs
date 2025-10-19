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

#[derive(Debug, Clone, Copy)]
pub struct AttributeWeight {
    pub weight: f32,
    pub name: &'static str,
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

impl PlayerAttributes {
    pub fn attribute_value(&self, name: &str) -> Option<u8> {
        match self {
            PlayerAttributes::Field {
                mental,
                physical,
                technical,
            } => match name {
                "vision" => Some(mental.vision),
                "composure" => Some(mental.composure),
                "positioning" => Some(mental.positioning),
                "pace" => Some(physical.pace),
                "stamina" => Some(physical.stamina),
                "strength" => Some(physical.strength),
                "passing" => Some(technical.passing),
                "heading" => Some(technical.heading),
                "tackling" => Some(technical.tackling),
                "dribbling" => Some(technical.dribbling),
                "finishing" => Some(technical.finishing),
                _ => None,
            },
            PlayerAttributes::Goalkeeper {
                mental,
                physical,
                technical,
            } => match name {
                "vision" => Some(mental.vision),
                "composure" => Some(mental.composure),
                "positioning" => Some(mental.positioning),
                "pace" => Some(physical.pace),
                "stamina" => Some(physical.stamina),
                "strength" => Some(physical.strength),
                "kicking" => Some(technical.kicking),
                "handling" => Some(technical.handling),
                "reflexes" => Some(technical.reflexes),
                "aerial_reach" => Some(technical.aerial_reach),
                _ => None,
            },
        }
    }

    pub fn mental(&self) -> &MentalAttributes {
        match self {
            PlayerAttributes::Field { mental, .. } => mental,
            PlayerAttributes::Goalkeeper { mental, .. } => mental,
        }
    }

    pub fn physical(&self) -> &PhysicalAttributes {
        match self {
            PlayerAttributes::Field { physical, .. } => physical,
            PlayerAttributes::Goalkeeper { physical, .. } => physical,
        }
    }
}
