use crate::events::{Death, EventHandlers};

pub fn event_handlers() -> EventHandlers {
    EventHandlers {
        implemented: true,
        take_damage: Some(|this, take_damage, game| {
            if take_damage.target == this && take_damage.amount == 1 {
                game.push_event(Death::new(this, this).into());
            }
        }),
        ..Default::default()
    }
}

pub fn golden_event_handlers() -> EventHandlers {
    EventHandlers {
        implemented: true,
        take_damage: Some(|this, take_damage, game| {
            if take_damage.target == this && take_damage.amount == 2 {
                game.push_event(Death::new(this, this).into());
            }
        }),
        ..Default::default()
    }
}
