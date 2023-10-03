use super::super::MinionVariantData;
#[allow(unused_imports)]
use crate::minions::{AbilitiesInit, MinionType};
use tinyvec::array_vec;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Annoy-o-Tron"),
        attack: 1u8,
        health: 2u8,
        attack_golden: 2u8,
        health_golden: 4u8,
        abilities: AbilitiesInit {
            taunt: true,
            shield: true,
            ..Default::default()
        }
            .init(),
        minion_types: array_vec![_ => MinionType::Mech,],
    }
}
