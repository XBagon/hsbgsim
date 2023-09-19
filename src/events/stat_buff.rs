use crate::MinionInstanceId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct StatBuff {
    pub target: MinionInstanceId,
    pub attack: i32,
    pub health: i32,
}

impl StatBuff {
    pub fn new(target: MinionInstanceId, attack: i32, health: i32) -> Self {
        Self {
            target,
            attack,
            health,
        }
    }
}
