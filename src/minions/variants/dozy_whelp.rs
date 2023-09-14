use crate::events::{EventHandler, StatBuff};

pub fn event_handler() -> EventHandler {
    EventHandler {
        propose_attack: Some(|this, attack, game| {
            if attack.defender == this {
                game.push_event(StatBuff::new(this, 1, 0).into());
            }
        }),
        ..Default::default()
    }
}
