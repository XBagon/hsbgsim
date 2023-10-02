use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Mistake"),
        attack: 1u8,
        health: 3u8,
        attack_golden: 2u8,
        health_golden: 6u8,
        abilities: AbilitiesInit {
            ..Default::default()
        }
            .init(),
    }
}
