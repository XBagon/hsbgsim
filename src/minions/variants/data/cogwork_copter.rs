use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Cogwork Copter"),
        attack: 1u8,
        health: 1u8,
        attack_golden: 2u8,
        health_golden: 2u8,
        abilities: AbilitiesInit {
            shield: true,
            ..Default::default()
        }
            .init(),
    }
}
