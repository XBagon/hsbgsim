use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Stormbringer"),
        attack: 1u8,
        health: 9u8,
        attack_golden: 2u8,
        health_golden: 18u8,
        abilities: AbilitiesInit {
            ..Default::default()
        }
            .init(),
    }
}
