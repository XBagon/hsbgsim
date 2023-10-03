use super::super::MinionVariantData;
#[allow(unused_imports)]
use crate::minions::{AbilitiesInit, MinionType};
use tinyvec::array_vec;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Glowscale"),
        attack: 4u8,
        health: 6u8,
        attack_golden: 8u8,
        health_golden: 12u8,
        abilities: AbilitiesInit {
            taunt: true,
            spellcraft: true,
            ..Default::default()
        }
            .init(),
        minion_types: array_vec![_ => MinionType::Naga,],
    }
}
