use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Baltharak"),
        attack: 10u8,
        health: 1u8,
        attack_golden: 20u8,
        health_golden: 2u8,
        abilities: AbilitiesInit {
            reborn: true,
            ..Default::default()
        }
            .init(),
    }
}
