use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Annoy-o-Module"),
        health: 4u8,
        attack: 2u8,
        abilities: AbilitiesInit {
            shield: true,
            magnetic: true,
            ..Default::default()
        }
            .init(),
    }
}
