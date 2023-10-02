use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Waverider"),
        attack: 2u8,
        health: 8u8,
        attack_golden: 4u8,
        health_golden: 16u8,
        abilities: AbilitiesInit {
            spellcraft: true,
            ..Default::default()
        }
            .init(),
    }
}
