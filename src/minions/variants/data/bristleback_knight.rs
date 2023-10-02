use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Bristleback Knight"),
        attack: 6u8,
        health: 9u8,
        attack_golden: 12u8,
        health_golden: 18u8,
        abilities: AbilitiesInit {
            shield: true,
            windfury: true,
            ..Default::default()
        }
            .init(),
    }
}
