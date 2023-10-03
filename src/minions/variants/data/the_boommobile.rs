use super::super::MinionVariantData;
#[allow(unused_imports)]
use crate::minions::{AbilitiesInit, MinionType};
use tinyvec::array_vec;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("The Boommobile"),
        attack: 12u8,
        health: 12u8,
        attack_golden: 24u8,
        health_golden: 24u8,
        abilities: AbilitiesInit {
            taunt: true,
            shield: true,
            windfury: true,
            reborn: true,
            magnetic: true,
            ..Default::default()
        }
            .init(),
        minion_types: array_vec![_ => MinionType::Mech,],
    }
}
