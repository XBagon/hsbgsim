use crate::{minions::Position, MinionInstanceId};

#[derive(Clone, Copy, Debug)]
pub struct Summon {
    pub minion: MinionInstanceId,
    pub position: Position,
}

impl Summon {
    pub fn new(minion: MinionInstanceId, position: Position) -> Self {
        Self {
            minion,
            position,
        }
    }
}
