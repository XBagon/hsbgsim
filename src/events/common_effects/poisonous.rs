use crate::{events::TakeDamage, Game, MinionInstanceId};

pub fn poisonous(this: MinionInstanceId, take_damage: TakeDamage, game: &mut Game) {
    if take_damage.source == this {
        game.minion_instances.get_mut(take_damage.target).unwrap().pending_destroy = true;
    }
}
