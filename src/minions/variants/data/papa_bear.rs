use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Papa Bear"),
        attack: 12u8,
        health: 12u8,
        attack_golden: 24u8,
        health_golden: 24u8,
        abilities: AbilitiesInit {
            deathrattle: true,
            ..Default::default()
        }
            .init(),
    }
}
