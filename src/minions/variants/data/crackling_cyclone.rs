use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Crackling Cyclone"),
        attack: 4u8,
        health: 1u8,
        attack_golden: 8u8,
        health_golden: 2u8,
        abilities: AbilitiesInit {
            shield: true,
            windfury: true,
            ..Default::default()
        }
            .init(),
    }
}
