use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Captain Flat Tusk"),
        health: 6u8,
        attack: 9u8,
        abilities: AbilitiesInit {
            ..Default::default()
        }
            .init(),
    }
}
