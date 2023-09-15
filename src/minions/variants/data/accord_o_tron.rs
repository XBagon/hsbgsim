use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Accord-o-Tron"),
        health: 3u8,
        attack: 3u8,
        abilities: AbilitiesInit {
            magnetic: true,
            ..Default::default()
        }
            .init(),
    }
}
