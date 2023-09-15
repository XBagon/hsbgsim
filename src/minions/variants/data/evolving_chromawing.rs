use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Evolving Chromawing"),
        health: 4u8,
        attack: 1u8,
        abilities: AbilitiesInit {
            ..Default::default()
        }
            .init(),
    }
}
