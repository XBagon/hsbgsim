use super::super::MinionVariantData;
#[allow(unused_imports)]
use crate::minions::{AbilitiesInit, MinionType};
use tinyvec::array_vec;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Baltharak"),
        attack: 10u8,
        health: 1u8,
        attack_golden: 20u8,
        health_golden: 2u8,
        abilities: AbilitiesInit {
            reborn: true,
            ..Default::default()
        }
            .init(),
        minion_types: array_vec![_ => MinionType::Mech, MinionType::Demon,],
    }
}
