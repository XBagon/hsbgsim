use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Dazzling Lightspawn"),
        attack: 2u8,
        health: 5u8,
        attack_golden: 4u8,
        health_golden: 10u8,
        abilities: AbilitiesInit {
            avenge: true,
            ..Default::default()
        }
            .init(),
    }
}
