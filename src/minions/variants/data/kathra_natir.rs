use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Kathra'natir"),
        health: 4u8,
        attack: 5u8,
        abilities: AbilitiesInit {
            ..Default::default()
        }
            .init(),
    }
}
