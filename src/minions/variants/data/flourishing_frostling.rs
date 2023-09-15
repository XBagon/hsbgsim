use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Flourishing Frostling"),
        health: 6u8,
        attack: 0u8,
        abilities: AbilitiesInit {
            ..Default::default()
        }
            .init(),
    }
}
