use super::super::MinionVariantData;
#[allow(unused_imports)]
use crate::minions::{AbilitiesInit, MinionType};
use tinyvec::array_vec;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Screwjank Clunker"),
        attack: 2u8,
        health: 5u8,
        attack_golden: 4u8,
        health_golden: 10u8,
        abilities: AbilitiesInit {
            battlecry: true,
            ..Default::default()
        }
            .init(),
        minion_types: array_vec![_ => MinionType::Mech,],
    }
}
