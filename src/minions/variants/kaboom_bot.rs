use crate::events::{EventHandlers, TakeDamage};
use rand::seq::SliceRandom;

pub fn event_handlers() -> EventHandlers {
    EventHandlers {
        death: Some(|this, death, game| {
            if death.minion == this {
                let this_minion = game.minion_instances.get(this).unwrap();
                let player_id = this_minion.position.unwrap().player_id;
                let pool: Vec<_> = game
                    .battleground
                    .player(player_id.oppsite())
                    .board
                    .minions
                    .iter()
                    .filter(|&&minion| {
                        let minion = game.minion_instances.get(minion).unwrap();
                        !minion.pending_destroy && minion.health > 0
                    })
                    .collect();
                if let Some(&&minion) = pool.choose(&mut game.rng) {
                    game.push_event(TakeDamage::new(minion, 4, this).into());
                }
            }
        }),
        ..Default::default()
    }
}
