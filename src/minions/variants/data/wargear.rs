use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Wargear"),
        attack: 6u8,
        health: 5u8,
        attack_golden: 12u8,
        health_golden: 10u8,
        abilities: AbilitiesInit {
            magnetic: true,
            ..Default::default()
        }
            .init(),
    }
}
