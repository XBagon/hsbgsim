use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Risen Rider"),
        health: 1u8,
        attack: 2u8,
        abilities: AbilitiesInit {
            taunt: true,
            reborn: true,
            ..Default::default()
        }
            .init(),
    }
}
