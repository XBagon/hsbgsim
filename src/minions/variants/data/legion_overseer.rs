use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Legion Overseer"),
        attack: 4u8,
        health: 2u8,
        attack_golden: 8u8,
        health_golden: 4u8,
        abilities: AbilitiesInit {
            ..Default::default()
        }
            .init(),
    }
}
