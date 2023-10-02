use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Mecha-Jaraxxus"),
        attack: 3u8,
        health: 15u8,
        attack_golden: 6u8,
        health_golden: 30u8,
        abilities: AbilitiesInit {
            battlecry: true,
            ..Default::default()
        }
            .init(),
    }
}
