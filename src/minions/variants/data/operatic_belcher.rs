use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Operatic Belcher"),
        health: 2u8,
        attack: 5u8,
        abilities: AbilitiesInit {
            deathrattle: true,
            venomous: true,
            ..Default::default()
        }
            .init(),
    }
}
