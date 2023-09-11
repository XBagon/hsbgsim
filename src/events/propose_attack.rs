use crate::MinionInstanceId;

use super::Attack;

#[derive(Clone, Copy, Debug)]
pub struct ProposeAttack {
    pub attacker: MinionInstanceId,
    pub defender: MinionInstanceId,
}

impl ProposeAttack {
    pub fn new(attacker: MinionInstanceId, defender: MinionInstanceId) -> Self {
        Self {
            attacker,
            defender,
        }
    }
}
