use super::super::MinionVariantData;
#[allow(unused_imports)]
use crate::minions::{AbilitiesInit, MinionType};
use tinyvec::array_vec;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Seaborn Summoner"),
        attack: 2u8,
        health: 4u8,
        attack_golden: 4u8,
        health_golden: 8u8,
        abilities: AbilitiesInit {
            spellcraft: true,
            ..Default::default()
        }
            .init(),
        minion_types: array_vec![_ => MinionType::Elemental, MinionType::Naga,],
    }
}
