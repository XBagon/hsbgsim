use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Insatiable Ur'zul"),
        attack: 4u8,
        health: 6u8,
        attack_golden: 8u8,
        health_golden: 12u8,
        abilities: AbilitiesInit {
            taunt: true,
            ..Default::default()
        }
            .init(),
    }
}
