use rand::seq::SliceRandom;

use super::{Abilities, MinionInstance};
use crate::events::EventHandler;

mod data;
automod::dir!(pub "src/minions/variants");

include!(concat!(env!("OUT_DIR"), "/variants.rs"));

pub struct MinionVariantData {
    pub name: String,
    pub health: u8,
    pub attack: u8,
    pub abilities: Abilities,
}

impl MinionVariant {
    pub fn into_instance(self) -> MinionInstance {
        let data = self.data();
        MinionInstance {
            variant: self,
            health: data.health as i32,
            attack: data.attack as i32,
            abilities: data.abilities,
            position: None,
            event_handler: self.event_handler(),
        }
    }
}

impl Default for MinionVariant {
    fn default() -> Self {
        MinionVariant::Invalid
    }
}
