use crate::MinionInstanceId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct TakeDamage {
    pub target: MinionInstanceId,
    pub amount: i32,
    pub source: MinionInstanceId,
}

impl TakeDamage {
    pub fn new(target: MinionInstanceId, amount: i32, source: MinionInstanceId) -> Self {
        Self {
            target,
            amount,
            source,
        }
    }
}
