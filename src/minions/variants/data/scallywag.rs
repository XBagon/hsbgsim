use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Scallywag"),
        attack: 3u8,
        health: 1u8,
        attack_golden: 6u8,
        health_golden: 2u8,
        abilities: AbilitiesInit {
            deathrattle: true,
            ..Default::default()
        }
            .init(),
    }
}
