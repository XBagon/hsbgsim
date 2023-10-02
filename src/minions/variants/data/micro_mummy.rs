use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Micro Mummy"),
        attack: 1u8,
        health: 2u8,
        attack_golden: 2u8,
        health_golden: 4u8,
        abilities: AbilitiesInit {
            reborn: true,
            ..Default::default()
        }
            .init(),
    }
}
