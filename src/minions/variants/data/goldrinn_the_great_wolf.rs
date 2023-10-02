use super::super::MinionVariantData;
use crate::minions::AbilitiesInit;
pub fn data() -> MinionVariantData {
    MinionVariantData {
        name: String::from("Goldrinn, the Great Wolf"),
        attack: 4u8,
        health: 4u8,
        attack_golden: 8u8,
        health_golden: 8u8,
        abilities: AbilitiesInit {
            deathrattle: true,
            ..Default::default()
        }
            .init(),
    }
}
