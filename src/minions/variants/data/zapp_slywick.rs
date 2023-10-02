use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Zapp Slywick"),
        attack: 8u8,
        health: 16u8,
        attack_golden: 16u8,
        health_golden: 32u8,
        abilities: AbilitiesInit {
            windfury: true,
            ..Default::default()
        }
            .init(),
    }
}
