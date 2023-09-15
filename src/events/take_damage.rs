use crate::MinionInstanceId;

#[derive(Clone, Copy, Debug)]
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
