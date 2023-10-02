use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Polarizing Beatboxer"),
        attack: 5u8,
        health: 10u8,
        attack_golden: 10u8,
        health_golden: 20u8,
        abilities: AbilitiesInit {
            ..Default::default()
        }
            .init(),
    }
}
