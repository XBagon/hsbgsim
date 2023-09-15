use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Scarlet Skull"),
        health: 1u8,
        attack: 2u8,
        abilities: AbilitiesInit {
            deathrattle: true,
            reborn: true,
            ..Default::default()
        }
            .init(),
    }
}
