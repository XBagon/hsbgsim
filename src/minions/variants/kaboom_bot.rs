use crate::{
    events::{AssociatedEventHandler, Death, EventHandlers, RawActivateEffect, TakeDamage},
    Game, MinionInstanceId,
};
use rand::seq::SliceRandom;

pub fn event_handlers() -> EventHandlers {
    EventHandlers {
        implemented: true,
        death: Some(deathrattle),
        ..Default::default()
    }
}

pub fn golden_event_handlers() -> EventHandlers {
    EventHandlers {
        implemented: true,
        death: Some(|this, death, game| {
            game.push_event(
                RawActivateEffect::from_event_and_handler(
                    death,
                    AssociatedEventHandler::<Death>::new(this, deathrattle),
                )
                .into(),
            );
            deathrattle(this, death, game);
        }),
        ..Default::default()
    }
}

pub fn deathrattle(this: MinionInstanceId, death: Death, game: &mut Game) {
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
                !minion.pending_destroy && minion.health() > 0
            })
            .collect();
        if let Some(&&minion) = pool.choose(&mut game.rng) {
            game.push_event(TakeDamage::new(minion, 4, this).into());
        }
    }
}
