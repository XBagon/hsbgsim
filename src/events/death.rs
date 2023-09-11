use crate::MinionInstanceId;

use super::Attack;

#[derive(Clone, Copy, Debug)]
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
