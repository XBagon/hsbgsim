use crate::MinionInstanceId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Death {
    pub minion: MinionInstanceId,
}

impl Death {
    pub fn new(minion: MinionInstanceId) -> Self {
        Self {
            minion,
        }
    }
}
