use crate::{events::EventHandlers, minions::MinionType};
pub fn event_handlers() -> EventHandlers {
    EventHandlers {
        implemented: true,
        aura_update: Some(|this, _, game| {
            let owner = game.minion_instances.get(this).unwrap().position.unwrap_board().player_id;
            game.foreach_minion_of(owner, |mi_id, minion| {
                if this != mi_id && minion.variant.data().minion_types.contains(&MinionType::Murloc)
                {
                    minion.aura_attack += 2;
                }
            })
        }),
        ..Default::default()
    }
}
pub fn golden_event_handlers() -> EventHandlers {
    EventHandlers {
        implemented: true,
        aura_update: Some(|this, _, game| {
            let owner = game.minion_instances.get(this).unwrap().position.unwrap_board().player_id;
            game.foreach_minion_of(owner, |mi_id, minion| {
                if this != mi_id && minion.variant.data().minion_types.contains(&MinionType::Murloc)
                {
                    minion.aura_attack += 4;
                }
            })
        }),
        ..Default::default()
    }
}
