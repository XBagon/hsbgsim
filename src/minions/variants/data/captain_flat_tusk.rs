use super::super::MinionVariantData;
#[allow(unused_imports)]
use crate::minions::{AbilitiesInit, MinionType};
use tinyvec::array_vec;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Captain Flat Tusk"),
        attack: 9u8,
        health: 6u8,
        attack_golden: 18u8,
        health_golden: 12u8,
        abilities: AbilitiesInit {
            ..Default::default()
        }
            .init(),
        minion_types: array_vec![_ => MinionType::Quilboar,],
    }
}
