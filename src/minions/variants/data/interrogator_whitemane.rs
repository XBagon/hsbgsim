use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Interrogator Whitemane"),
        health: 5u8,
        attack: 8u8,
        abilities: AbilitiesInit {
            ..Default::default()
        }
            .init(),
    }
}