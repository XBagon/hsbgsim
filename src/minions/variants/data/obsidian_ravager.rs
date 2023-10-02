use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Obsidian Ravager"),
        attack: 7u8,
        health: 7u8,
        attack_golden: 14u8,
        health_golden: 14u8,
        abilities: AbilitiesInit {
            ..Default::default()
        }
            .init(),
    }
}
