use super::super::MinionVariantData;
#[allow(unused_imports)]
use crate::minions::{AbilitiesInit, MinionType};
use tinyvec::array_vec;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Elemental of Surprise"),
        attack: 8u8,
        health: 8u8,
        attack_golden: 16u8,
        health_golden: 16u8,
        abilities: AbilitiesInit {
            shield: true,
            ..Default::default()
        }
            .init(),
        minion_types: array_vec![_ => MinionType::Elemental,],
    }
}
