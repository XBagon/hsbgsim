use super::super::MinionVariantData;
#[allow(unused_imports)]
use crate::minions::{AbilitiesInit, MinionType};
use tinyvec::array_vec;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Tavern Tipper"),
        attack: 2u8,
        health: 2u8,
        attack_golden: 4u8,
        health_golden: 4u8,
        abilities: AbilitiesInit {
            ..Default::default()
        }
            .init(),
        minion_types: array_vec![_ =>],
    }
}
