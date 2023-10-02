use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Peggy Brittlebone"),
        attack: 6u8,
        health: 5u8,
        attack_golden: 12u8,
        health_golden: 10u8,
        abilities: AbilitiesInit {
            ..Default::default()
        }
            .init(),
    }
}
