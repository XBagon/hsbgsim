use crate::events::EventHandlers;
pub fn event_handlers() -> EventHandlers {
    EventHandlers {
        implemented: true,
        ..Default::default()
    }
}
pub fn golden_event_handlers() -> EventHandlers {
    EventHandlers {
        implemented: true,
        ..Default::default()
    }
}
