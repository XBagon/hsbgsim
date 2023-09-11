use crate::MinionInstanceId;

use super::Attack;

#[derive(Clone, Copy, Debug)]
pub struct AfterAttack {
    pub attacker: MinionInstanceId,
    pub defender: MinionInstanceId,
}

impl AfterAttack {
    pub fn new(attacker: MinionInstanceId, defender: MinionInstanceId) -> Self {
        Self {
            attacker,
            defender,
        }
    }
}

impl From<Attack> for AfterAttack {
    fn from(value: Attack) -> Self {
        Self {
            attacker: value.attacker,
            defender: value.defender,
        }
    }
}
