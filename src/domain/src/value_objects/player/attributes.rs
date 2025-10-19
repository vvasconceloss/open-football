use crate::value_objects::player::weights::Attribute;

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
    pub attribute: &'static Attribute,
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

impl AttributeWeight {
    pub fn new(attribute: &'static Attribute, weight: f32) -> Self {
        Self { attribute, weight }
    }
}

impl PlayerAttributes {
    pub fn get_attribute_value(&self, attribute: &Attribute) -> Option<u8> {
        match self {
            PlayerAttributes::Field {
                mental,
                physical,
                technical,
            } => match attribute {
                Attribute::Vision => Some(mental.vision),
                Attribute::Composure => Some(mental.composure),
                Attribute::Positioning => Some(mental.positioning),
                Attribute::Pace => Some(physical.pace),
                Attribute::Stamina => Some(physical.stamina),
                Attribute::Strength => Some(physical.strength),
                Attribute::Passing => Some(technical.passing),
                Attribute::Heading => Some(technical.heading),
                Attribute::Tackling => Some(technical.tackling),
                Attribute::Dribbling => Some(technical.dribbling),
                Attribute::Finishing => Some(technical.finishing),
                _ => None,
            },
            PlayerAttributes::Goalkeeper {
                mental,
                physical,
                technical,
            } => match attribute {
                Attribute::Vision => Some(mental.vision),
                Attribute::Composure => Some(mental.composure),
                Attribute::Positioning => Some(mental.positioning),
                Attribute::Pace => Some(physical.pace),
                Attribute::Stamina => Some(physical.stamina),
                Attribute::Strength => Some(physical.strength),
                Attribute::Kicking => Some(technical.kicking),
                Attribute::Handling => Some(technical.handling),
                Attribute::Reflexes => Some(technical.reflexes),
                Attribute::AerialReach => Some(technical.aerial_reach),
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
