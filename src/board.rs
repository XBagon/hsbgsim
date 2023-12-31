use crate::minions::MinionInstanceId;

use tinyvec::ArrayVec;

#[derive(Default, Debug, Clone)]
pub struct Board {
    pub minions: ArrayVec<[MinionInstanceId; 7]>,
}

impl Board {
    pub fn new(minions: ArrayVec<[MinionInstanceId; 7]>) -> Self {
        Self {
            minions,
        }
    }
}
