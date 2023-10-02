use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Moon-Bacon Jazzer"),
        attack: 2u8,
        health: 5u8,
        attack_golden: 4u8,
        health_golden: 10u8,
        abilities: AbilitiesInit {
            battlecry: true,
            ..Default::default()
        }
            .init(),
    }
}
