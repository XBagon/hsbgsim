use crate::events::EventHandlers;
//Note: This implementation will interact weirdly with `TitusRivendare`
pub fn event_handlers() -> EventHandlers {
    EventHandlers {
        implemented: true,
        aura_update: Some(|this, _, game| {
            let player_id =
                game.minion_instances.get(this).unwrap().position.unwrap_board().player_id;
            game.battleground.player_mut(player_id).extra_deathrattle_count = 1;
        }),
        ..Default::default()
    }
}
pub fn golden_event_handlers() -> EventHandlers {
    EventHandlers {
        implemented: true,
        aura_update: Some(|this, _, game| {
            let player_id =
                game.minion_instances.get(this).unwrap().position.unwrap_board().player_id;
            game.battleground.player_mut(player_id).extra_deathrattle_count = 2;
        }),
        ..Default::default()
    }
}
