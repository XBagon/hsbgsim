use crate::{
    events::{Attack, TakeDamage},
    Game, MinionInstanceId,
};

pub fn cleave(this: MinionInstanceId, attack: Attack, game: &mut Game) {
    if attack.attacker == this {
        let attack_stat = game.minion_instances.get(attack.attacker).unwrap().attack;
        let defender = game.minion_instances.get(attack.defender).unwrap();
        let position = defender.position.unwrap_board();

        let mut left_position = *position;
        left_position.index = left_position.index.wrapping_sub(1);
        let mut right_position = *position;
        right_position.index = right_position.index.wrapping_add(1);

        if let Some(left_defender) = game.battleground.minion_at(left_position) {
            game.push_event(TakeDamage::new(left_defender, attack_stat, this).into());
        }
        if let Some(right_defender) = game.battleground.minion_at(right_position) {
            game.push_event(TakeDamage::new(right_defender, attack_stat, this).into());
        }
    }
}
