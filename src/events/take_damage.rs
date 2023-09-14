use crate::MinionInstanceId;

#[derive(Clone, Copy, Debug)]
pub struct TakeDamage {
    pub target: MinionInstanceId,
    pub amount: i32,
}

impl TakeDamage {
    pub fn new(target: MinionInstanceId, amount: i32) -> Self {
        Self {
            target,
            amount,
        }
    }
}
