use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Greta Gold-Gun"),
        attack: 2u8,
        health: 9u8,
        attack_golden: 4u8,
        health_golden: 18u8,
        abilities: AbilitiesInit {
            spellcraft: true,
            ..Default::default()
        }
            .init(),
    }
}
