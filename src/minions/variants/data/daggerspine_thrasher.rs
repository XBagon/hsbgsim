use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Daggerspine Thrasher"),
        attack: 3u8,
        health: 5u8,
        attack_golden: 6u8,
        health_golden: 10u8,
        abilities: AbilitiesInit {
            ..Default::default()
        }
            .init(),
    }
}
