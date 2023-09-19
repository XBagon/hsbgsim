use super::ProposeAttack;
use crate::MinionInstanceId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Attack {
    pub attacker: MinionInstanceId,
    pub defender: MinionInstanceId,
    pub is_outer_phase: bool,
}

impl Attack {
    pub fn new(attacker: MinionInstanceId, defender: MinionInstanceId, outer_phase: bool) -> Self {
        Self {
            attacker,
            defender,
            is_outer_phase: outer_phase,
        }
    }
}

impl From<ProposeAttack> for Attack {
    fn from(value: ProposeAttack) -> Self {
        Self {
            attacker: value.attacker,
            defender: value.defender,
            is_outer_phase: value.is_outer_phase,
        }
    }
}
