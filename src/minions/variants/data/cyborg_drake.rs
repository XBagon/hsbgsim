use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Cyborg Drake"),
        attack: 3u8,
        health: 10u8,
        attack_golden: 6u8,
        health_golden: 20u8,
        abilities: AbilitiesInit {
            shield: true,
            ..Default::default()
        }
            .init(),
    }
}
