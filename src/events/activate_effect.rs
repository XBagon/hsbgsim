use serde::{Deserialize, Serialize};

use crate::Event;

use super::{AssociatedEventHandler, EventHandling, EventTrait, GenericEventHandling};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActivateEffect {
    pub generic_associated_event_handler: GenericEventHandling,
}

impl ActivateEffect {
    pub fn new(generic_associated_event_handler: GenericEventHandling) -> Self {
        Self {
            generic_associated_event_handler,
        }
    }

    pub fn from_event_and_handler<Ev: EventTrait>(
        event: Ev,
        ass_handler: AssociatedEventHandler<Ev>,
    ) -> Self
    where
        GenericEventHandling: From<EventHandling<Ev>>,
    {
        Self::new(EventHandling::new(event, ass_handler).into())
    }
}

impl From<ActivateEffect> for Event {
    fn from(value: ActivateEffect) -> Self {
        Event::ActivateEffect(Box::new(value))
    }
}
