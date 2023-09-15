use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Palescale Crocolisk"),
        health: 5u8,
        attack: 4u8,
        abilities: AbilitiesInit {
            deathrattle: true,
            avenge: true,
            ..Default::default()
        }
            .init(),
    }
}
