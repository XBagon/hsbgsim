use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Obsidian Ravager"),
        health: 7u8,
        attack: 7u8,
        abilities: AbilitiesInit {
            ..Default::default()
        }
            .init(),
    }
}
