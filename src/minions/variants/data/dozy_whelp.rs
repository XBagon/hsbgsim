use super::super::MinionVariantData;
#[allow(unused_imports)]
use crate::minions::{AbilitiesInit, MinionType};
use tinyvec::array_vec;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Dozy Whelp"),
        attack: 0u8,
        health: 3u8,
        attack_golden: 0u8,
        health_golden: 6u8,
        abilities: AbilitiesInit {
            taunt: true,
            ..Default::default()
        }
            .init(),
        minion_types: array_vec![_ => MinionType::Dragon,],
    }
}
