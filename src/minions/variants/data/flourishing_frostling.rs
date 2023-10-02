use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Flourishing Frostling"),
        attack: 0u8,
        health: 6u8,
        attack_golden: 0u8,
        health_golden: 12u8,
        abilities: AbilitiesInit {
            ..Default::default()
        }
            .init(),
    }
}
