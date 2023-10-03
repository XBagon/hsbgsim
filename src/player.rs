use crate::{board::Board, hand::Hand, hero::Hero, MinionInstanceId};
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct Player {
    hero: Hero,
    hand: Hand,
    pub board: Board,
    pub next_attack_position: u8,
    pub last_attacking_minion: MinionInstanceId,
    pub extra_deathrattle_count: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerId {
    Top,
    Bottom,
}

impl PlayerId {
    pub fn oppsite(&self) -> Self {
        match self {
            PlayerId::Top => PlayerId::Bottom,
            PlayerId::Bottom => PlayerId::Top,
        }
    }
}

impl Player {
    pub fn new(hero: Hero, hand: Hand, board: Board) -> Self {
        Self {
            hero,
            hand,
            board,
            next_attack_position: 0,
            last_attacking_minion: MinionInstanceId::default(),
            extra_deathrattle_count: 0,
        }
    }
}
