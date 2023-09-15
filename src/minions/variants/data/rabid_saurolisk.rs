use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Rabid Saurolisk"),
        health: 2u8,
        attack: 3u8,
        abilities: AbilitiesInit {
            ..Default::default()
        }
            .init(),
    }
}
