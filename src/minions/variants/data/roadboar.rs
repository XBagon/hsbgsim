use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Roadboar"),
        health: 4u8,
        attack: 2u8,
        abilities: AbilitiesInit {
            ..Default::default()
        }
            .init(),
    }
}