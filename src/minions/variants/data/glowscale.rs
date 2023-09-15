use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Glowscale"),
        health: 6u8,
        attack: 4u8,
        abilities: AbilitiesInit {
            taunt: true,
            spellcraft: true,
            ..Default::default()
        }
            .init(),
    }
}
