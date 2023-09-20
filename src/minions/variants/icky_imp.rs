use crate::events::{EventHandlers, Summon};

pub fn event_handlers() -> EventHandlers {
    EventHandlers {
        death: Some(|this, death, game| {
            if death.minion == this {
                let position = game.minion_instances.get(this).unwrap().position.unwrap();
                let imp1 = game.instantiate_minion(crate::MinionVariant::Imp);
                let imp2 = game.instantiate_minion(crate::MinionVariant::Imp);
                game.push_event(Summon::new(imp1, position).into());
                game.push_event(Summon::new(imp2, position).into());
            }
        }),
        ..Default::default()
    }
}
