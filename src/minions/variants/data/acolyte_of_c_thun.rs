use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Acolyte of C'Thun"),
        attack: 2u8,
        health: 3u8,
        attack_golden: 4u8,
        health_golden: 6u8,
        abilities: AbilitiesInit {
            taunt: true,
            reborn: true,
            ..Default::default()
        }
            .init(),
    }
}
