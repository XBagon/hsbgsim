use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Mantid Queen"),
        attack: 5u8,
        health: 5u8,
        attack_golden: 10u8,
        health_golden: 10u8,
        abilities: AbilitiesInit {
            venomous: true,
            ..Default::default()
        }
            .init(),
    }
}
