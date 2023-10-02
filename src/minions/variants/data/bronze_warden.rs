use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Bronze Warden"),
        attack: 2u8,
        health: 1u8,
        attack_golden: 4u8,
        health_golden: 2u8,
        abilities: AbilitiesInit {
            shield: true,
            reborn: true,
            ..Default::default()
        }
            .init(),
    }
}
