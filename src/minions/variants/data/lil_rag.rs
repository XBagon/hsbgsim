use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Lil' Rag"),
        health: 6u8,
        attack: 6u8,
        abilities: AbilitiesInit {
            ..Default::default()
        }
            .init(),
    }
}
