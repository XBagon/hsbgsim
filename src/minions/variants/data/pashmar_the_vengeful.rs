use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Pashmar the Vengeful"),
        attack: 4u8,
        health: 5u8,
        attack_golden: 8u8,
        health_golden: 10u8,
        abilities: AbilitiesInit {
            avenge: true,
            ..Default::default()
        }
            .init(),
    }
}
