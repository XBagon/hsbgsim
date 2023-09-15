use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Bonker"),
        health: 7u8,
        attack: 3u8,
        abilities: AbilitiesInit {
            windfury: true,
            ..Default::default()
        }
            .init(),
    }
}
