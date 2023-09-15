use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Sanguine Champion"),
        health: 3u8,
        attack: 18u8,
        abilities: AbilitiesInit {
            battlecry: true,
            deathrattle: true,
            ..Default::default()
        }
            .init(),
    }
}
