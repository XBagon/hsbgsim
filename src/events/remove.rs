use crate::MinionInstanceId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Remove {
    pub minion: MinionInstanceId,
}

impl Remove {
    pub fn new(minion: MinionInstanceId) -> Self {
        Self {
            minion,
        }
    }
}
