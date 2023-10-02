use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Snail Cavalry"),
        attack: 5u8,
        health: 2u8,
        attack_golden: 10u8,
        health_golden: 4u8,
        abilities: AbilitiesInit {
            ..Default::default()
        }
            .init(),
    }
}
