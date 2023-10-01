use crate::events::{common_effects::cleave, EventHandlers};

pub fn event_handlers() -> EventHandlers {
    EventHandlers {
        attack: Some(cleave),
        ..Default::default()
    }
}
