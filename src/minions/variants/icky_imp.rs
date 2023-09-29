use crate::events::{EventHandlers, Summon};

pub fn event_handlers() -> EventHandlers {
    EventHandlers {
        death: Some(|this, death, game| {
            if death.minion == this {
                let position = game.minion_instances.get(this).unwrap().position;
                let imp1 = game.instantiate_minion(crate::MinionVariant::Imp);
                let imp2 = game.instantiate_minion(crate::MinionVariant::Imp);
                game.push_events(&[
                    Summon::new(imp1, *position.unwrap(), this).into(),
                    Summon::new(imp2, *position.unwrap(), this).into(),
                ]);
            }
        }),
        ..Default::default()
    }
}
