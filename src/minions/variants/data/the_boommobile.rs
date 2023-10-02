use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("The Boommobile"),
        attack: 12u8,
        health: 12u8,
        attack_golden: 24u8,
        health_golden: 24u8,
        abilities: AbilitiesInit {
            taunt: true,
            shield: true,
            windfury: true,
            reborn: true,
            magnetic: true,
            ..Default::default()
        }
            .init(),
    }
}
