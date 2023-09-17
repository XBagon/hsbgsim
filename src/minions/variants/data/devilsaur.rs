use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Devilsaur"),
        health: 8u8,
        attack: 8u8,
        abilities: AbilitiesInit {
            ..Default::default()
        }
            .init(),
    }
}