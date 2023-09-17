use slotmap::new_key_type;

use super::MinionVariant;
use crate::{events::EventHandler, minions::Abilities, player::PlayerId};

new_key_type! {
    pub struct MinionInstanceId;
}

#[derive(Default)]
pub struct MinionInstance {
    pub variant: MinionVariant,
    pub health: i32,
    pub attack: i32,
    pub position: Option<Position>,
    pub abilities: Abilities,
    pub event_handler: EventHandler,
}

impl MinionInstance {
    pub fn stats_print(&self) -> String {
        let mut attack = format!("{}", self.attack);
        if self.abilities.venomous() {
            attack = format!("v{}", attack);
        }
        let mut health = format!("{}", self.health);
        if self.abilities.shield() {
            health = format!("({})", health);
        };
        if self.abilities.taunt() {
            health = format!("[{}]", health);
        }

        format!("{}/{}", attack, health)
    }
}

#[derive(Clone, Copy)]
pub struct Position {
    pub player_id: PlayerId,
    pub index: u8,
}

impl Position {
    pub fn new(player_id: PlayerId, index: u8) -> Self {
        debug_assert!(index < 7);
        Self {
            player_id,
            index,
        }
    }
}
