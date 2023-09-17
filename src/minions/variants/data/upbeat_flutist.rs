use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Upbeat Flutist"),
        health: 5u8,
        attack: 2u8,
        abilities: AbilitiesInit {
            ..Default::default()
        }
            .init(),
    }
}