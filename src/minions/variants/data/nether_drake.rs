use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Nether Drake"),
        health: 5u8,
        attack: 1u8,
        abilities: AbilitiesInit {
            ..Default::default()
        }
            .init(),
    }
}
