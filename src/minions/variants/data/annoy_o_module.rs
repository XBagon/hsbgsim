use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Annoy-o-Module"),
        attack: 2u8,
        health: 4u8,
        attack_golden: 4u8,
        health_golden: 8u8,
        abilities: AbilitiesInit {
            shield: true,
            magnetic: true,
            ..Default::default()
        }
            .init(),
    }
}
