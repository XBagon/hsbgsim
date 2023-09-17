use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Soulsplitter"),
        health: 2u8,
        attack: 4u8,
        abilities: AbilitiesInit {
            reborn: true,
            ..Default::default()
        }
            .init(),
    }
}