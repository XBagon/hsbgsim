use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Deflect-o-Bot"),
        attack: 3u8,
        health: 2u8,
        attack_golden: 6u8,
        health_golden: 4u8,
        abilities: AbilitiesInit {
            shield: true,
            ..Default::default()
        }
            .init(),
    }
}
