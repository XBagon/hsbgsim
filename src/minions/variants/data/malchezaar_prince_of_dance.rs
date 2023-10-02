use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Malchezaar, Prince of Dance"),
        attack: 6u8,
        health: 3u8,
        attack_golden: 12u8,
        health_golden: 6u8,
        abilities: AbilitiesInit {
            ..Default::default()
        }
            .init(),
    }
}
