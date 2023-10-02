use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Octosari, Wrap God"),
        attack: 6u8,
        health: 7u8,
        attack_golden: 12u8,
        health_golden: 14u8,
        abilities: AbilitiesInit {
            deathrattle: true,
            ..Default::default()
        }
            .init(),
    }
}
