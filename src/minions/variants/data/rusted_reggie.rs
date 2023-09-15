use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Rusted Reggie"),
        health: 5u8,
        attack: 5u8,
        abilities: AbilitiesInit {
            windfury: true,
            ..Default::default()
        }
            .init(),
    }
}
