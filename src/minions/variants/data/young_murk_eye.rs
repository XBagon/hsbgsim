use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Young Murk-Eye"),
        attack: 8u8,
        health: 5u8,
        attack_golden: 16u8,
        health_golden: 10u8,
        abilities: AbilitiesInit {
            ..Default::default()
        }
            .init(),
    }
}
