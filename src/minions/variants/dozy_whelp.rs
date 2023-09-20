use crate::events::{EventHandlers, StatBuff};

pub fn event_handlers() -> EventHandlers {
    EventHandlers {
        propose_attack: Some(|this, attack, game| {
            if attack.defender == this {
                game.push_event(StatBuff::new(this, 1, 0).into());
            }
        }),
        ..Default::default()
    }
}
