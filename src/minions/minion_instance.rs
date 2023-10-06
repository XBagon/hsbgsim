use super::MinionVariant;
use crate::{events::EventHandlers, minions::Abilities, player::PlayerId};
use serde::{Deserialize, Serialize};
use slotmap::new_key_type;

new_key_type! {
    pub struct MinionInstanceId;
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct MinionInstance {
    pub variant: MinionVariant,
    pub golden: bool,
    pub base_attack: i32,
    pub base_health: i32,
    pub aura_attack: i32,
    pub aura_health: i32,
    pub position: Position,
    pub abilities: Abilities,
    pub pending_destroy: bool,
    pub event_handlers: EventHandlers,
}

impl MinionInstance {
    pub fn attack(&self) -> i32 {
        self.base_attack + self.aura_attack
    }

    pub fn health(&self) -> i32 {
        self.base_health + self.aura_health
    }

    pub fn stats_print(&self) -> String {
        let mut attack = format!("{}", self.attack());
        if self.abilities.venomous() {
            attack = format!("v{}", attack);
        }
        if self.abilities.venomous() {
            attack = format!("v{}", attack);
        }
        if self.abilities.windfury() {
            attack = format!("w{}", attack);
        }
        let mut health = format!("{}", self.health());
        if self.abilities.shield() {
            health = format!("({})", health);
        };
        if self.abilities.taunt() {
            health = format!("[{}]", health);
        }

        format!("{}/{}", attack, health)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoardPosition {
    pub player_id: PlayerId,
    pub index: u8,
}

impl BoardPosition {
    pub fn new(player_id: PlayerId, index: u8) -> Self {
        Self {
            player_id,
            index,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Position {
    BoardPosition(BoardPosition),
    LastPosition(Option<BoardPosition>),
}

impl Position {
    pub fn new_on_board(player_id: PlayerId, index: u8) -> Self {
        Position::BoardPosition(BoardPosition::new(player_id, index))
    }

    pub fn is_on_board(&self) -> bool {
        match self {
            Position::BoardPosition(_) => true,
            Position::LastPosition(_) => false,
        }
    }

    pub fn unwrap_board(&self) -> &BoardPosition {
        match self {
            Position::BoardPosition(pos) => pos,
            Position::LastPosition(_) => {
                panic!("called `Position::unwrap_board()` on a `LastPosition` value")
            }
        }
    }

    pub fn unwrap_board_mut(&mut self) -> &mut BoardPosition {
        match self {
            Position::BoardPosition(pos) => pos,
            Position::LastPosition(_) => {
                panic!("called `Position::unwrap_board_mut()` on a `LastPosition` value")
            }
        }
    }

    pub fn unwrap(&self) -> &BoardPosition {
        match self {
            Position::BoardPosition(pos) => pos,
            Position::LastPosition(pos) => pos.as_ref().unwrap(),
        }
    }

    pub fn unwrap_mut(&mut self) -> &mut BoardPosition {
        match self {
            Position::BoardPosition(pos) => pos,
            Position::LastPosition(pos) => pos.as_mut().unwrap(),
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Position::LastPosition(None)
    }
}
