use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Eternal Summoner"),
        attack: 8u8,
        health: 1u8,
        attack_golden: 16u8,
        health_golden: 2u8,
        abilities: AbilitiesInit {
            deathrattle: true,
            reborn: true,
            ..Default::default()
        }
            .init(),
    }
}
