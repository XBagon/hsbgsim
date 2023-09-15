use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Sin'dorei Straight Shot"),
        health: 4u8,
        attack: 3u8,
        abilities: AbilitiesInit {
            shield: true,
            windfury: true,
            ..Default::default()
        }
            .init(),
    }
}
