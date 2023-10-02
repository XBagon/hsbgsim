use crate::events::common_effects::poisonous;
use crate::events::EventHandlers;

pub fn event_handlers() -> EventHandlers {
    EventHandlers {
        implemented: true,
        take_damage: Some(poisonous),
        ..Default::default()
    }
}

pub fn golden_event_handlers() -> EventHandlers {
    EventHandlers {
        implemented: true,
        take_damage: Some(poisonous),
        ..Default::default()
    }
}
