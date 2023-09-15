use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Whelp Smuggler"),
        health: 5u8,
        attack: 2u8,
        abilities: AbilitiesInit {
            ..Default::default()
        }
            .init(),
    }
}
