use crate::{
    minions::{BoardPosition, MinionInstanceId},
    player::{Player, PlayerId},
};

#[derive(Default, Clone)]
pub struct Battleground {
    bottom_player: Player,
    top_player: Player,
}

impl Battleground {
    pub fn new(bottom_player: Player, top_player: Player) -> Self {
        Self {
            bottom_player,
            top_player,
        }
    }

    pub fn player(&self, id: PlayerId) -> &Player {
        match id {
            PlayerId::Bottom => &self.bottom_player,
            PlayerId::Top => &self.top_player,
        }
    }

    pub fn player_mut(&mut self, player_id: PlayerId) -> &mut Player {
        match player_id {
            PlayerId::Bottom => &mut self.bottom_player,
            PlayerId::Top => &mut self.top_player,
        }
    }

    pub fn players(&self) -> [&Player; 2] {
        [&self.bottom_player, &self.top_player]
    }

    pub fn players_mut(&mut self) -> [&mut Player; 2] {
        [&mut self.bottom_player, &mut self.top_player]
    }

    //TODO: replace player(player_id) -> minions[index] calls with this
    pub fn minion_at(&self, position: BoardPosition) -> Option<MinionInstanceId> {
        self.player(position.player_id).board.minions.get(position.index as usize).copied()
    }

    pub fn all_minions(&self) -> impl Iterator<Item = MinionInstanceId> + '_ {
        self.bottom_player
            .board
            .minions
            .iter()
            .copied()
            .chain(self.top_player.board.minions.iter().copied())
    }
}
