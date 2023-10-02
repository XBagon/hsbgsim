use crate::events::{EventHandlers, Summon};

pub fn event_handlers() -> EventHandlers {
    EventHandlers {
        implemented: true,
        death: Some(|this, death, game| {
            if death.minion == this {
                let position = game.minion_instances.get(this).unwrap().position;
                let damaged_golem =
                    game.instantiate_minion(crate::MinionVariant::DamagedGolem, false);
                game.push_event(Summon::new(damaged_golem, *position.unwrap(), this).into());
            }
        }),
        ..Default::default()
    }
}

pub fn golden_event_handlers() -> EventHandlers {
    EventHandlers {
        implemented: true,
        death: Some(|this, death, game| {
            if death.minion == this {
                let position = game.minion_instances.get(this).unwrap().position;
                let damaged_golem =
                    game.instantiate_minion(crate::MinionVariant::DamagedGolem, true);
                game.push_event(Summon::new(damaged_golem, *position.unwrap(), this).into());
            }
        }),
        ..Default::default()
    }
}
