use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Magtheridon Prime"),
        attack: 1u8,
        health: 10u8,
        attack_golden: 2u8,
        health_golden: 20u8,
        abilities: AbilitiesInit {
            taunt: true,
            ..Default::default()
        }
            .init(),
    }
}
