use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Emergent Flame"),
        attack: 5u8,
        health: 3u8,
        attack_golden: 10u8,
        health_golden: 6u8,
        abilities: AbilitiesInit {
            battlecry: true,
            ..Default::default()
        }
            .init(),
    }
}
