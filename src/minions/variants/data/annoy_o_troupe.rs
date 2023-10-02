use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Annoy-o-Troupe"),
        attack: 3u8,
        health: 6u8,
        attack_golden: 6u8,
        health_golden: 12u8,
        abilities: AbilitiesInit {
            deathrattle: true,
            taunt: true,
            shield: true,
            ..Default::default()
        }
            .init(),
    }
}
