use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Golden Monkey"),
        attack: 6u8,
        health: 6u8,
        attack_golden: 12u8,
        health_golden: 12u8,
        abilities: AbilitiesInit {
            taunt: true,
            ..Default::default()
        }
            .init(),
    }
}
