use crate::events::{EventHandlers, StatBuff};

pub fn event_handlers() -> EventHandlers {
    EventHandlers {
        implemented: true,
        propose_attack: Some(|this, attack, game| {
            if attack.defender == this {
                game.push_event(StatBuff::new(this, 1, 0).into());
            }
        }),
        ..Default::default()
    }
}

pub fn golden_event_handlers() -> EventHandlers {
    EventHandlers {
        implemented: true,
        propose_attack: Some(|this, attack, game| {
            if attack.defender == this {
                game.push_event(StatBuff::new(this, 2, 0).into());
            }
        }),
        ..Default::default()
    }
}
