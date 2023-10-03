use super::super::MinionVariantData;
#[allow(unused_imports)]
use crate::minions::{AbilitiesInit, MinionType};
use tinyvec::array_vec;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Mecha-Jaraxxus"),
        attack: 3u8,
        health: 15u8,
        attack_golden: 6u8,
        health_golden: 30u8,
        abilities: AbilitiesInit {
            battlecry: true,
            ..Default::default()
        }
            .init(),
        minion_types: array_vec![_ => MinionType::Mech, MinionType::Demon,],
    }
}
