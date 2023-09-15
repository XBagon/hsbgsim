use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("The Boommobile"),
        health: 12u8,
        attack: 12u8,
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
