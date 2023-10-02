use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Reanimating Rattler"),
        attack: 4u8,
        health: 3u8,
        attack_golden: 8u8,
        health_golden: 6u8,
        abilities: AbilitiesInit {
            battlecry: true,
            ..Default::default()
        }
            .init(),
    }
}
