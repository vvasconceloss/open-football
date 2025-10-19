use std::collections::HashMap;

use crate::value_objects::player::attributes::PlayerAttributes;

#[derive(Debug, Clone)]
pub struct PlayerAbility {
    pub current: f32,   //Current Ability (CA)
    pub potential: f32, //Evolutionary Potential (EP)
    pub impact: f32,    //Contextual Impact Index (CII)
}

impl PlayerAbility {
    pub fn calculate(
        attributes: &PlayerAttributes,
        weights: &HashMap<&'static str, f32>,
        player_age: u8,
        tactic_context: f32,
        consistency_training: f32,
    ) -> Self {
        let mental_attrs = attributes.mental();
        let physical_attrs = attributes.physical();

        let (
            passing,
            heading,
            tackling,
            dribbling,
            finishing,
            kicking,
            handling,
            reflexes,
            aerial_reach,
        ) = match attributes {
            PlayerAttributes::Field { technical, .. } => (
                technical.passing,
                technical.heading,
                technical.tackling,
                technical.dribbling,
                technical.finishing,
                0,
                0,
                0,
                0,
            ),
            PlayerAttributes::Goalkeeper { technical, .. } => (
                0,
                0,
                0,
                0,
                0,
                technical.kicking,
                technical.reflexes,
                technical.handling,
                technical.aerial_reach,
            ),
        };

        let weighted_average = |pairs: Vec<(f32, f32)>| -> f32 {
            let sum_weights: f32 = pairs.iter().map(|(_, weight)| weight).sum();
            let total: f32 = pairs.iter().map(|(value, weight)| value * weight).sum();

            if sum_weights > 0.0 {
                total / sum_weights
            } else {
                0.0
            }
        };

        let technical_average = weighted_average(vec![
            (passing as f32, *weights.get("passing").unwrap_or(&0.0)),
            (heading as f32, *weights.get("heading").unwrap_or(&0.0)),
            (kicking as f32, *weights.get("kicking").unwrap_or(&0.0)),
            (handling as f32, *weights.get("handling").unwrap_or(&0.0)),
            (reflexes as f32, *weights.get("reflexes").unwrap_or(&0.0)),
            (tackling as f32, *weights.get("tackling").unwrap_or(&0.0)),
            (finishing as f32, *weights.get("finishing").unwrap_or(&0.0)),
            (dribbling as f32, *weights.get("dribbling").unwrap_or(&0.0)),
            (
                aerial_reach as f32,
                *weights.get("aerial_reach").unwrap_or(&0.0),
            ),
        ]);

        let mental_average = weighted_average(vec![
            (
                mental_attrs.vision as f32,
                *weights.get("vision").unwrap_or(&0.0),
            ),
            (
                mental_attrs.composure as f32,
                *weights.get("composure").unwrap_or(&0.0),
            ),
            (
                mental_attrs.positioning as f32,
                *weights.get("positioning").unwrap_or(&0.0),
            ),
        ]);

        let physical_average = weighted_average(vec![
            (
                physical_attrs.pace as f32,
                *weights.get("pace").unwrap_or(&0.0),
            ),
            (
                physical_attrs.stamina as f32,
                *weights.get("stamina").unwrap_or(&0.0),
            ),
            (
                physical_attrs.strength as f32,
                *weights.get("strength").unwrap_or(&0.0),
            ),
        ]);

        let phys_dominance = physical_average * (physical_attrs.stamina as f32 / 100.0);
        let tech_consistency = technical_average * (1.0 + mental_attrs.composure as f32 / 100.0);
        let game_reading =
            mental_average * ((mental_attrs.vision + mental_attrs.positioning) as f32 / 200.0);

        let age_factor = ((27.0 - player_age as f32).max(0.0)) / 27.0;
        let current_ability = 0.6 * tech_consistency + 0.3 * game_reading + 0.1 * phys_dominance; // CA
        let evolutionary_potential =
            (0.4 * game_reading + 0.2 * phys_dominance + 0.4 * age_factor * 100.0).min(100.0);

        let contextual_impact_index = (current_ability * tactic_context
            + evolutionary_potential * consistency_training)
            / (tactic_context + consistency_training);

        Self {
            current: current_ability,
            potential: evolutionary_potential,
            impact: contextual_impact_index,
        }
    }
}
