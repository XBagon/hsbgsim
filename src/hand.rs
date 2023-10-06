use tinyvec::ArrayVec;

use crate::MinionInstance;

#[derive(Default, Clone)]
pub struct Hand {
    minions: ArrayVec<[MinionInstance; 10]>,
}

impl Hand {
    pub fn new(minions: ArrayVec<[MinionInstance; 10]>) -> Self {
        Self {
            minions,
        }
    }
}
