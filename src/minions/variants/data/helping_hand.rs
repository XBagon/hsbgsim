use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Helping Hand"),
        health: 1u8,
        attack: 2u8,
        abilities: AbilitiesInit {
            reborn: true,
            ..Default::default()
        }
            .init(),
    }
}