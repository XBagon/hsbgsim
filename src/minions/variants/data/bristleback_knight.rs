use super::super::MinionVariantData;
#[allow(unused_imports)]
use crate::minions::{AbilitiesInit, MinionType};
use tinyvec::array_vec;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Bristleback Knight"),
        attack: 6u8,
        health: 9u8,
        attack_golden: 12u8,
        health_golden: 18u8,
        abilities: AbilitiesInit {
            shield: true,
            windfury: true,
            ..Default::default()
        }
            .init(),
        minion_types: array_vec![_ => MinionType::Quilboar,],
    }
}
