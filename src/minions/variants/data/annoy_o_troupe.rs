use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Annoy-o-Troupe"),
        health: 6u8,
        attack: 3u8,
        abilities: AbilitiesInit {
            deathrattle: true,
            taunt: true,
            shield: true,
            ..Default::default()
        }
            .init(),
    }
}
