use crate::events::EventHandler;
use rand::seq::SliceRandom;

pub fn event_handler() -> EventHandler {
    EventHandler {
        death: Some(|this, death, game| {
            if death.minion == this {
                let this_minion = game.minion_instances.get(this).unwrap();
                let player_id = this_minion.position.unwrap().player_id;
                let pool: Vec<_> = game
                    .battleground
                    .player(player_id)
                    .board
                    .minions
                    .iter()
                    .filter(|&&minion| {
                        let minion = game.minion_instances.get(minion).unwrap();
                        !minion.pending_destroy && minion.health > 0 && !minion.abilities.shield()
                    })
                    .collect();
                if let Some(&&minion) = pool.choose(&mut game.rng) {
                    game.minion_instances.get_mut(minion).unwrap().abilities.set_shield(true);
                }
            }
        }),
        ..Default::default()
    }
}
