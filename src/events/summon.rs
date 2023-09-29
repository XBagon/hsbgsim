use crate::{minions::BoardPosition, MinionInstanceId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Summon {
    pub minion: MinionInstanceId,
    pub position: BoardPosition,
    pub source: MinionInstanceId,
}

impl Summon {
    pub fn new(
        minion: MinionInstanceId,
        position: BoardPosition,
        source: MinionInstanceId,
    ) -> Self {
        Self {
            minion,
            position,
            source,
        }
    }
}
