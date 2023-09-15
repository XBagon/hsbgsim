use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Sparring Partner"),
        health: 2u8,
        attack: 3u8,
        abilities: AbilitiesInit {
            battlecry: true,
            taunt: true,
            ..Default::default()
        }
            .init(),
    }
}
