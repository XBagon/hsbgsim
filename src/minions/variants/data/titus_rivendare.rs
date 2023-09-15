use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Titus Rivendare"),
        health: 7u8,
        attack: 1u8,
        abilities: AbilitiesInit {
            ..Default::default()
        }
            .init(),
    }
}
