use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Lullabot"),
        health: 2u8,
        attack: 2u8,
        abilities: AbilitiesInit {
            magnetic: true,
            ..Default::default()
        }
            .init(),
    }
}
