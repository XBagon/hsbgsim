use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Xylo-bones"),
        attack: 6u8,
        health: 1u8,
        attack_golden: 12u8,
        health_golden: 2u8,
        abilities: AbilitiesInit {
            ..Default::default()
        }
            .init(),
    }
}
