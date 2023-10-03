use super::super::MinionVariantData;
#[allow(unused_imports)]
use crate::minions::{AbilitiesInit, MinionType};
use tinyvec::array_vec;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Tide Oracle Morgl"),
        attack: 1u8,
        health: 10u8,
        attack_golden: 2u8,
        health_golden: 20u8,
        abilities: AbilitiesInit {
            venomous: true,
            ..Default::default()
        }
            .init(),
        minion_types: array_vec![_ => MinionType::Murloc,],
    }
}
