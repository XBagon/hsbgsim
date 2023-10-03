use super::super::MinionVariantData;
#[allow(unused_imports)]
use crate::minions::{AbilitiesInit, MinionType};
use tinyvec::array_vec;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Octosari, Wrap God"),
        attack: 6u8,
        health: 7u8,
        attack_golden: 12u8,
        health_golden: 14u8,
        abilities: AbilitiesInit {
            deathrattle: true,
            ..Default::default()
        }
            .init(),
        minion_types: array_vec![_ => MinionType::Beast,],
    }
}
