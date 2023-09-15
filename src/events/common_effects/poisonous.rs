use crate::{
    events::{Death, TakeDamage},
    Game, MinionInstanceId,
};

pub fn poisonous(this: MinionInstanceId, take_damage: TakeDamage, game: &mut Game) {
    if take_damage.source == this {
        game.push_event(Death::new(take_damage.target).into());
    }
}
