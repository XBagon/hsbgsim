use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("SI:Sefin"),
        attack: 2u8,
        health: 6u8,
        attack_golden: 4u8,
        health_golden: 12u8,
        abilities: AbilitiesInit {
            avenge: true,
            ..Default::default()
        }
            .init(),
    }
}
