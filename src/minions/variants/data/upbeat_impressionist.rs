use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Upbeat Impressionist"),
        attack: 8u8,
        health: 4u8,
        attack_golden: 16u8,
        health_golden: 8u8,
        abilities: AbilitiesInit {
            ..Default::default()
        }
            .init(),
    }
}
