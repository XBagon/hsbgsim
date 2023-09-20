use crate::events::common_effects::poisonous;
use crate::events::EventHandlers;

pub fn event_handlers() -> EventHandlers {
    EventHandlers {
        take_damage: Some(poisonous),
        ..Default::default()
    }
}
