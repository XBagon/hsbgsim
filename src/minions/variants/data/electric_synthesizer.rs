use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Electric Synthesizer"),
        attack: 3u8,
        health: 6u8,
        attack_golden: 6u8,
        health_golden: 12u8,
        abilities: AbilitiesInit {
            battlecry: true,
            ..Default::default()
        }
            .init(),
    }
}
