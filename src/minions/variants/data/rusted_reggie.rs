use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Rusted Reggie"),
        attack: 5u8,
        health: 5u8,
        attack_golden: 10u8,
        health_golden: 10u8,
        abilities: AbilitiesInit {
            windfury: true,
            ..Default::default()
        }
            .init(),
    }
}
