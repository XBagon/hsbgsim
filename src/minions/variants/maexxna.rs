use crate::events::common_effects::poisonous;
use crate::events::EventHandler;

pub fn event_handler() -> EventHandler {
    EventHandler {
        take_damage: Some(poisonous),
        ..Default::default()
    }
}
