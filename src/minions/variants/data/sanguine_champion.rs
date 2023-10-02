use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Sanguine Champion"),
        attack: 18u8,
        health: 3u8,
        attack_golden: 36u8,
        health_golden: 6u8,
        abilities: AbilitiesInit {
            battlecry: true,
            deathrattle: true,
            ..Default::default()
        }
            .init(),
    }
}
