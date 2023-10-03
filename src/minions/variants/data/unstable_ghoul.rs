use super::super::MinionVariantData;
#[allow(unused_imports)]
use crate::minions::{AbilitiesInit, MinionType};
use tinyvec::array_vec;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Unstable Ghoul"),
        attack: 1u8,
        health: 3u8,
        attack_golden: 2u8,
        health_golden: 6u8,
        abilities: AbilitiesInit {
            deathrattle: true,
            taunt: true,
            ..Default::default()
        }
            .init(),
        minion_types: array_vec![_ =>],
    }
}
