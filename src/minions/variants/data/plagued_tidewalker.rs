use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Plagued Tidewalker"),
        attack: 2u8,
        health: 7u8,
        attack_golden: 4u8,
        health_golden: 14u8,
        abilities: AbilitiesInit {
            venomous: true,
            ..Default::default()
        }
            .init(),
    }
}
