use super::super::MinionVariantData;
#[allow(unused_imports)]
use crate::minions::{AbilitiesInit, MinionType};
use tinyvec::array_vec;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Zapp Slywick"),
        attack: 8u8,
        health: 16u8,
        attack_golden: 16u8,
        health_golden: 32u8,
        abilities: AbilitiesInit {
            windfury: true,
            ..Default::default()
        }
            .init(),
        minion_types: array_vec![_ =>],
    }
}
