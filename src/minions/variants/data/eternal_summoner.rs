use super::super::MinionVariantData;
#[allow(unused_imports)]
use crate::minions::{AbilitiesInit, MinionType};
use tinyvec::array_vec;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Eternal Summoner"),
        attack: 8u8,
        health: 1u8,
        attack_golden: 16u8,
        health_golden: 2u8,
        abilities: AbilitiesInit {
            deathrattle: true,
            reborn: true,
            ..Default::default()
        }
            .init(),
        minion_types: array_vec![_ => MinionType::Undead,],
    }
}
