use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Colossus of the Sun"),
        health: 6u8,
        attack: 6u8,
        abilities: AbilitiesInit {
            shield: true,
            reborn: true,
            ..Default::default()
        }
            .init(),
    }
}
