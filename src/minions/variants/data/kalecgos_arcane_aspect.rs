use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Kalecgos, Arcane Aspect"),
        attack: 2u8,
        health: 10u8,
        attack_golden: 4u8,
        health_golden: 20u8,
        abilities: AbilitiesInit {
            ..Default::default()
        }
            .init(),
    }
}
