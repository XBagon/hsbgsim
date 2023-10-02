use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Keyboard Igniter"),
        attack: 4u8,
        health: 1u8,
        attack_golden: 8u8,
        health_golden: 2u8,
        abilities: AbilitiesInit {
            battlecry: true,
            ..Default::default()
        }
            .init(),
    }
}
