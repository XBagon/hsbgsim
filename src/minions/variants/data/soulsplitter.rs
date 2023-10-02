use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Soulsplitter"),
        attack: 4u8,
        health: 2u8,
        attack_golden: 8u8,
        health_golden: 4u8,
        abilities: AbilitiesInit {
            reborn: true,
            ..Default::default()
        }
            .init(),
    }
}
