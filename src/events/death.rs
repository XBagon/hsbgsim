use crate::MinionInstanceId;



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
