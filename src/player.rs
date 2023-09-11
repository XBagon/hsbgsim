use crate::{board::Board, hand::Hand, hero::Hero};

#[derive(Default)]
pub struct Player {
    hero: Hero,
    hand: Hand,
    pub board: Board,
}

#[derive(Clone, Copy)]
pub enum PlayerId {
    Top,
    Bottom,
}

impl Player {
    pub fn new(hero: Hero, hand: Hand, board: Board) -> Self {
        Self {
            hero,
            hand,
            board,
        }
    }
}
