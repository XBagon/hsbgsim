use super::super::MinionVariantData;
#[allow(unused_imports)]
use crate::minions::{AbilitiesInit, MinionType};
use tinyvec::array_vec;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Cyborg Drake"),
        attack: 3u8,
        health: 10u8,
        attack_golden: 6u8,
        health_golden: 20u8,
        abilities: AbilitiesInit {
            shield: true,
            ..Default::default()
        }
            .init(),
        minion_types: array_vec![_ => MinionType::Mech, MinionType::Dragon,],
    }
}
