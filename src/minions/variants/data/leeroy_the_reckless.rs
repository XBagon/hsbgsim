use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Leeroy the Reckless"),
        attack: 6u8,
        health: 2u8,
        attack_golden: 12u8,
        health_golden: 4u8,
        abilities: AbilitiesInit {
            deathrattle: true,
            ..Default::default()
        }
            .init(),
    }
}
