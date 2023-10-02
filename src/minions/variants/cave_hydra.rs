use crate::events::{common_effects::cleave, EventHandlers};

pub fn event_handlers() -> EventHandlers {
    EventHandlers {
        implemented: true,
        attack: Some(cleave),
        ..Default::default()
    }
}

pub fn golden_event_handlers() -> EventHandlers {
    EventHandlers::default()
}
