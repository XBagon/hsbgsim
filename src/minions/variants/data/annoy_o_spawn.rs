use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Annoy-o-Spawn"),
        health: 2u8,
        attack: 1u8,
        abilities: AbilitiesInit {
            taunt: true,
            shield: true,
            ..Default::default()
        }
            .init(),
    }
}
