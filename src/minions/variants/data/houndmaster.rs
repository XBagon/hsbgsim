use super::super::MinionVariantData;
#[allow(unused_imports)]
use crate::minions::{AbilitiesInit, MinionType};
use tinyvec::array_vec;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Houndmaster"),
        attack: 4u8,
        health: 3u8,
        attack_golden: 8u8,
        health_golden: 6u8,
        abilities: AbilitiesInit {
            battlecry: true,
            ..Default::default()
        }
            .init(),
        minion_types: array_vec![_ =>],
    }
}
