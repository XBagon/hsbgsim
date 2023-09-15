use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Wargear"),
        health: 5u8,
        attack: 6u8,
        abilities: AbilitiesInit {
            magnetic: true,
            ..Default::default()
        }
            .init(),
    }
}
