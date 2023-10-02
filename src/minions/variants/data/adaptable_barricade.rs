use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Adaptable Barricade"),
        attack: 18u8,
        health: 1u8,
        attack_golden: 36u8,
        health_golden: 2u8,
        abilities: AbilitiesInit {
            taunt: true,
            ..Default::default()
        }
            .init(),
    }
}
