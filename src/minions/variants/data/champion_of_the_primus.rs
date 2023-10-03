use super::super::MinionVariantData;
#[allow(unused_imports)]
use crate::minions::{AbilitiesInit, MinionType};
use tinyvec::array_vec;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Champion of the Primus"),
        attack: 2u8,
        health: 9u8,
        attack_golden: 4u8,
        health_golden: 18u8,
        abilities: AbilitiesInit {
            avenge: true,
            ..Default::default()
        }
            .init(),
        minion_types: array_vec![_ => MinionType::Undead,],
    }
}
