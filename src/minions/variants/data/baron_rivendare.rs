use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Baron Rivendare"),
        attack: 1u8,
        health: 7u8,
        attack_golden: 2u8,
        health_golden: 14u8,
        abilities: AbilitiesInit {
            ..Default::default()
        }
            .init(),
    }
}
