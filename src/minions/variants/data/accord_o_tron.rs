use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Accord-o-Tron"),
        attack: 3u8,
        health: 3u8,
        attack_golden: 6u8,
        health_golden: 6u8,
        abilities: AbilitiesInit {
            magnetic: true,
            ..Default::default()
        }
            .init(),
    }
}
