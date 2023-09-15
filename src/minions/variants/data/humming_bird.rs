use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Humming Bird"),
        health: 5u8,
        attack: 0u8,
        abilities: AbilitiesInit {
            ..Default::default()
        }
            .init(),
    }
}
