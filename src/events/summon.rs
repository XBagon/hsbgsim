use crate::{minions::Position, MinionInstanceId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
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
