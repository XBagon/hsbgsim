use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Replicating Menace"),
        health: 2u8,
        attack: 3u8,
        abilities: AbilitiesInit {
            deathrattle: true,
            magnetic: true,
            ..Default::default()
        }
            .init(),
    }
}
