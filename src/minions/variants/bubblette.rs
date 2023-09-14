use crate::events::{Death, EventHandler};

pub fn event_handler() -> EventHandler {
    EventHandler {
        take_damage: Some(|this, take_damage, game| {
            if take_damage.target == this && take_damage.amount == 1 {
                game.push_event(Death::new(this).into());
            }
        }),
        ..Default::default()
    }
}
