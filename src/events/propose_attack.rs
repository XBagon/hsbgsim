use crate::MinionInstanceId;

#[derive(Clone, Copy, Debug)]
pub struct ProposeAttack {
    pub attacker: MinionInstanceId,
    pub defender: MinionInstanceId,
    pub is_outer_phase: bool,
}

impl ProposeAttack {
    pub fn new(attacker: MinionInstanceId, defender: MinionInstanceId, outer_phase: bool) -> Self {
        Self {
            attacker,
            defender,
            is_outer_phase: outer_phase,
        }
    }
}
