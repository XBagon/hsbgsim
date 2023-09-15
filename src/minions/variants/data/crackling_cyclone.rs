use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Crackling Cyclone"),
        health: 1u8,
        attack: 4u8,
        abilities: AbilitiesInit {
            shield: true,
            windfury: true,
            ..Default::default()
        }
            .init(),
    }
}
