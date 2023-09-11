use crate::MinionInstanceId;

use super::ProposeAttack;

#[derive(Clone, Copy, Debug)]
pub struct Attack {
    pub attacker: MinionInstanceId,
    pub defender: MinionInstanceId,
}

impl Attack {
    pub fn new(attacker: MinionInstanceId, defender: MinionInstanceId) -> Self {
        Self {
            attacker,
            defender,
        }
    }
}

impl From<ProposeAttack> for Attack {
    fn from(value: ProposeAttack) -> Self {
        Self {
            attacker: value.attacker,
            defender: value.defender,
        }
    }
}
