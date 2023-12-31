use crate::{
    events::{
        AuraUpdate, Death, DeathCheck, End, Event, EventHandlerManager, Events, ProposeAttack,
        RawActivateEffect, Remove, TakeDamage,
    },
    minions::{BoardPosition, MinionInstanceId, MinionVariant, Position},
    player::PlayerId,
    Battleground, MinionInstance,
};

use rand::prelude::*;
use rand_xoshiro::Xoshiro256PlusPlus;
use slotmap::{Key, SecondaryMap, SlotMap};
use thiserror::Error;
use tinyvec::ArrayVec;

type MinionInstances = SlotMap<MinionInstanceId, MinionInstance>;

pub struct Game {
    pub battleground: Battleground,
    pub minion_instances: MinionInstances,
    //TODO: minion_datas: SecondarySlotMap<MinionInstanceId, MinionData>,
    minion_summon_shift: SecondaryMap<MinionInstanceId, u8>,
    attacking_player: Option<PlayerId>,
    last_attacker: MinionInstanceId,
    additional_attacks: u8,
    events: Events,
    event_handler_manager: EventHandlerManager,
    pub(crate) rng: Xoshiro256PlusPlus,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            battleground: Default::default(),
            minion_instances: Default::default(),
            minion_summon_shift: SecondaryMap::new(),
            attacking_player: Default::default(),
            last_attacker: Default::default(),
            additional_attacks: Default::default(),
            events: Default::default(),
            event_handler_manager: Default::default(),
            rng: Xoshiro256PlusPlus::from_entropy(),
        }
    }
}

struct GameBuilder {}

impl GameBuilder {
    pub fn new() -> Self {
        Self {}
    }
}

impl Game {
    pub fn with_seed(seed: u64) -> Self {
        Self {
            battleground: Default::default(),
            minion_instances: Default::default(),
            minion_summon_shift: SecondaryMap::new(),
            attacking_player: Default::default(),
            last_attacker: Default::default(),
            additional_attacks: Default::default(),
            events: Default::default(),
            event_handler_manager: Default::default(),
            rng: Xoshiro256PlusPlus::seed_from_u64(seed),
        }
    }

    // use `replay_step` instead of step to advance replay
    pub fn from_replay(mut replay: Replay) -> Self {
        replay.events.reverse();
        let queue = replay.events;
        Self {
            battleground: replay.initial_game_snapshot.initial_battleground,
            minion_instances: replay.initial_game_snapshot.minions,
            minion_summon_shift: SecondaryMap::new(),
            attacking_player: None,
            last_attacker: MinionInstanceId::default(),
            additional_attacks: 0,
            events: Events {
                queue,
            },
            event_handler_manager: EventHandlerManager::default(),
            rng: Xoshiro256PlusPlus::from_entropy(),
        }
    }

    ///Needs to be run once before [`Self::step`] is called
    pub fn initialize(&mut self) {
        let bottom_minion_count = self.battleground.player(PlayerId::Bottom).board.minions.len();
        let top_minion_count = self.battleground.player(PlayerId::Top).board.minions.len();
        let starting_player = match bottom_minion_count.cmp(&top_minion_count) {
            std::cmp::Ordering::Less => PlayerId::Top,
            std::cmp::Ordering::Equal => {
                if self.rng.gen() {
                    PlayerId::Top
                } else {
                    PlayerId::Bottom
                }
            }
            std::cmp::Ordering::Greater => PlayerId::Bottom,
        };
        self.attacking_player = Some(starting_player);
        self.push_outer_phase_events();
    }

    pub fn instantiate_minion(&mut self, variant: MinionVariant, golden: bool) -> MinionInstanceId {
        let minion_instance = variant.into_instance(golden);
        let mi_id = self.minion_instances.insert(minion_instance);
        let event_handlers = if golden {
            variant.golden_event_handlers()
        } else {
            variant.event_handlers()
        };
        self.event_handler_manager.append_event_handler(mi_id, &event_handlers);
        mi_id
    }

    #[inline]
    pub fn step(&mut self) -> Event {
        self.simulate_step::<false>()
    }

    #[inline]
    pub fn replay_step(&mut self) -> Event {
        self.simulate_step::<true>()
    }

    fn simulate_step<const REPLAY: bool>(&mut self) -> Event {
        #[cfg(any(debug_assertions, test))]
        self.assert_all_positions_are_correct();

        let next_event = self.events.next();
        let next_event = if let Some(next_event) = next_event {
            next_event
        } else if REPLAY {
            return Event::Invalid;
        } else {
            //Check if game is over
            let bottom_empty = self.battleground.player(PlayerId::Bottom).board.minions.is_empty();
            let top_empty = self.battleground.player(PlayerId::Top).board.minions.is_empty();
            match (bottom_empty, top_empty) {
                (true, true) => End::Draw.into(),
                (true, false) => End::TopWin.into(),
                (false, true) => End::BottomWin.into(),
                (false, false) => {
                    //Cleanup for next attack:
                    self.minion_summon_shift.clear();

                    let attacking_player_id = self.attacking_player.unwrap();
                    let next_player_id = attacking_player_id.oppsite();

                    let attack_info = if self.additional_attacks > 0 && {
                        self.additional_attacks -= 1;
                        let minion = self.minion_instances.get(self.last_attacker).unwrap();
                        minion.position.is_on_board() && minion.attack() > 0
                    } {
                        //Last minion attacks again
                        Some((self.last_attacker, attacking_player_id))
                    } else if let Some(attacker) = self.determine_next_attacker(attacking_player_id)
                    {
                        Some((attacker, next_player_id))
                    } else {
                        //Switch attacker
                        self.determine_next_attacker(next_player_id)
                            .map(|attacker| (attacker, attacking_player_id))
                    };

                    if let Some((attacker, next_player_id)) = attack_info {
                        let minion = self.minion_instances.get(attacker).unwrap();
                        if attacker != self.last_attacker && minion.abilities.windfury() {
                            self.additional_attacks = 1;
                        }
                        let event = ProposeAttack::new(
                            attacker,
                            self.random_defender(next_player_id),
                            true,
                        )
                        .into();
                        self.last_attacker = attacker;
                        self.attacking_player = Some(next_player_id);
                        event
                    } else {
                        //Noone has minions that can attack
                        End::Draw.into()
                    }
                }
            }
        };

        //macro instead of function because of `Game` mut + immut borrow

        macro_rules! handle_events {
            ($event:ident) => {
                for i in (0..self.event_handler_manager.$event.len()).rev() {
                    self.push_event(
                        RawActivateEffect::from_event_and_handler(
                            $event,
                            self.event_handler_manager.$event[i],
                        )
                        .into(),
                    );
                }
            };
        }

        match &next_event {
            Event::Invalid => unreachable!(),
            Event::End(_end) => {}
            &Event::ProposeAttack(propose_attack) => {
                self.push_event(Event::Attack(propose_attack.into()));
                if propose_attack.is_outer_phase {
                    self.push_outer_phase_events();
                }
                handle_events!(propose_attack);
            }
            &Event::Attack(attack) => {
                self.push_event(Event::AfterAttack(attack.into()));
                if attack.is_outer_phase {
                    self.push_outer_phase_events();
                }
                let [attacker, defender] = self
                    .minion_instances
                    .get_disjoint_mut([attack.attacker, attack.defender])
                    .unwrap();
                debug_assert_ne!(
                    attacker.position.unwrap_board().player_id,
                    defender.position.unwrap_board().player_id
                );
                let mut events = ArrayVec::<[Event; 2]>::new();
                if defender.attack() > 0 {
                    if attacker.abilities.shield() {
                        attacker.abilities.set_shield(false);
                    } else {
                        let event =
                            TakeDamage::new(attack.attacker, defender.attack(), attack.defender)
                                .into();
                        events.push(event);
                    }
                    if attacker.abilities.venomous() {
                        attacker.abilities.set_venomous(false);
                        defender.pending_destroy = true;
                    }
                }
                if attacker.attack() > 0 {
                    if defender.abilities.shield() {
                        defender.abilities.set_shield(false);
                    } else {
                        let event =
                            TakeDamage::new(attack.defender, attacker.attack(), attack.attacker)
                                .into();
                        events.push(event);
                    }
                    if defender.abilities.venomous() {
                        defender.abilities.set_venomous(false);
                        attacker.pending_destroy = true;
                    }
                }
                handle_events!(attack);
                events.into_iter().for_each(|event| self.push_event(event));
                //for i in 0..self.event_handler_manager.attack.len() {
                //    let AssociatedEventHandler {minion: mi_id, handler} = self.event_handler_manager.attack[i];
                //    handler(mi_id, attack, self)
                //}
            }
            &Event::AfterAttack(after_attack) => {
                let [_attacker, _defender] = self
                    .minion_instances
                    .get_disjoint_mut([after_attack.attacker, after_attack.defender])
                    .unwrap();
                if after_attack.is_outer_phase {
                    self.push_outer_phase_events();
                }
            }
            &Event::DeathCheck(_death_check) => {
                let _event_count = self.events.len();
                //TODO: DeathCheck order is not clear. it might be random? (see https://old.reddit.com/r/hearthstone/comments/l49xao/battlegrounds_how_does_deathrattle_ordering_work/) But on one side of the board should work left to right!
                for mi_id in self.battleground.all_minions() {
                    let minion = self.minion_instances.get(mi_id).unwrap();
                    if minion.health() <= 0 || minion.pending_destroy {
                        //TODO: replace `MinionInstanceId::null()` with actual sensible source. Maybe look at PastEvents
                        self.events.push(Death::new(mi_id, MinionInstanceId::null()).into());
                    }
                }
            }
            &Event::Death(death) => {
                for i in (0..self.event_handler_manager.death.len()).rev() {
                    let ass_handler = self.event_handler_manager.death[i];
                    let deathrattle_count = if death.minion == ass_handler.minion {
                        //if deathrattle
                        let player_id = self
                            .minion_instances
                            .get(death.minion)
                            .unwrap()
                            .position
                            .unwrap_board()
                            .player_id;
                        1 + self.battleground.player(player_id).extra_deathrattle_count
                    } else {
                        1
                    };
                    for _ in 0..deathrattle_count {
                        self.push_event(
                            RawActivateEffect::from_event_and_handler(
                                death,
                                self.event_handler_manager.death[i],
                            )
                            .into(),
                        );
                    }
                }
                //let minion = self.minion_instances.get(death.minion).unwrap();
                // if minion.abilities.reborn() {self.push_event(Summon::new(death.minion, minion.position));}
                self.push_event(Remove::new(death.minion).into());
            }
            &Event::Remove(remove) => {
                self.remove_minion(remove.minion).unwrap();
            }
            &Event::StatBuff(stat_buff) => {
                let target = self.minion_instances.get_mut(stat_buff.target).unwrap();
                target.base_attack += stat_buff.attack;
                target.base_health += stat_buff.health;
            }
            &Event::TakeDamage(take_damage) => {
                self.minion_instances.get_mut(take_damage.target).unwrap().base_health -=
                    take_damage.amount;
                handle_events!(take_damage);
            }
            &Event::Summon(summon) => {
                let shift = self.minion_summon_shift.entry(summon.source).unwrap().or_insert(0);
                *shift += 1;
                let mut position = summon.position;
                position.index += *shift - 1; //-1 to increase after summon
                _ = self.position_minion_at(summon.minion, position);
                self.push_event(AuraUpdate.into());
            }
            Event::ActivateEffect(activate_effect) => {
                activate_effect.generic_associated_event_handler.handle(self);
            }
            &Event::AuraUpdate(aura_update) => {
                for minion in self.battleground.all_minions() {
                    let minion = self.minion_instances.get_mut(minion).unwrap();
                    minion.aura_attack = 0;
                    minion.aura_health = 0;
                }
                for player in self.battleground.players_mut() {
                    player.extra_deathrattle_count = 0;
                }

                handle_events!(aura_update);
            }
        }

        next_event
    }

    fn push_outer_phase_events(&mut self) {
        self.push_events(&[AuraUpdate.into(), DeathCheck.into()]);
    }

    pub fn push_event(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn push_events(&mut self, events: &[Event]) {
        for event in events.iter().cloned().rev() {
            self.events.push(event);
        }
    }

    pub fn run(&mut self) -> End {
        loop {
            let current_event = self.step();

            if let Event::End(result) = current_event {
                return result;
            }
        }
    }

    pub fn game_snapshot(&mut self) -> GameSnapshot {
        GameSnapshot::new(self.minion_instances.clone(), self.battleground.clone())
    }

    pub fn board_snapshot(&mut self) -> BoardSnapshot {
        let mut bottom_board = ArrayVec::new();
        for mi_id in self.battleground.player(PlayerId::Bottom).board.minions {
            let minion = self.minion_instances.get(mi_id).unwrap();
            bottom_board.push(minion.clone());
        }

        let mut top_board = ArrayVec::new();
        for mi_id in self.battleground.player(PlayerId::Top).board.minions {
            let minion = self.minion_instances.get(mi_id).unwrap();
            top_board.push(minion.clone());
        }

        BoardSnapshot::new(bottom_board, top_board)
    }

    pub fn run_and_record(&mut self) -> Replay {
        let game_snapshot = self.game_snapshot();
        let mut events = Vec::new();

        loop {
            let current_event = self.step();

            if let Event::End(_) = current_event {
                events.push(current_event);
                return Replay::new(game_snapshot, events);
            }

            events.push(current_event);
        }
    }

    pub fn run_and_print(&mut self) {
        let mut last_print = String::new();

        loop {
            let current_event = self.step();
            let current_print = self.game_print();

            println!("{:-^128}", format!("{:?}", current_event));
            if current_print != last_print {
                println!("{}", self.game_print());
            }
            last_print = current_print;

            if let Event::End(_) = current_event {
                break;
            }
        }
    }

    pub fn determine_next_attacker(&mut self, player_id: PlayerId) -> Option<MinionInstanceId> {
        let attacking_player = self.battleground.player_mut(player_id);
        let minion_count = attacking_player.board.minions.len();

        for i in 0..minion_count {
            let next_position = (attacking_player.next_attack_position as usize + i) % minion_count;
            let next_mi_id = attacking_player.board.minions[next_position];
            let next_minion = self.minion_instances.get(next_mi_id).unwrap();
            if next_minion.attack() > 0 {
                attacking_player.next_attack_position = next_position as u8 + 1;
                return Some(next_mi_id);
            }
        }

        None
    }

    pub fn random_defender(&mut self, player_id: PlayerId) -> MinionInstanceId {
        let mut taunted_not_stealthed = Vec::new();
        let mut not_taunted_not_stealthed = Vec::new();
        for mi_id in self.battleground.player(player_id).board.minions {
            let minion = self.minion_instances.get(mi_id).unwrap();
            if !minion.abilities.stealth() {
                if minion.abilities.taunt() {
                    taunted_not_stealthed.push(mi_id);
                } else {
                    not_taunted_not_stealthed.push(mi_id);
                }
            }
        }
        let valid_targets = if taunted_not_stealthed.is_empty() {
            not_taunted_not_stealthed
        } else {
            taunted_not_stealthed
        };
        *valid_targets.choose(&mut self.rng).unwrap()
    }

    pub fn game_print(&self) -> String {
        let mut top_names = String::new();
        let mut top_stats = String::new();
        for minion_id in self.battleground.player(PlayerId::Top).board.minions {
            let minion = &self.minion_instances[minion_id];
            let mut name = format!(
                "{}{}|{:?}",
                minion.golden.then_some(String::from("G-")).unwrap_or_default(),
                minion.variant.data().name,
                minion_id.data()
            );
            if minion_id == self.last_attacker {
                name = format!("*{}*", name);
            }
            let stats = minion.stats_print();
            let width = name.len().max(stats.len()) + 2;
            top_names.push_str(&format!("{:^width$}", name));
            top_stats.push_str(&format!("{:^width$}", stats));
        }

        let mut bottom_names = String::new();
        let mut bottom_stats = String::new();
        for minion_id in self.battleground.player(PlayerId::Bottom).board.minions {
            let minion = &self.minion_instances[minion_id];
            let mut name = format!(
                "{}{}|{:?}",
                minion.golden.then_some(String::from("G-")).unwrap_or_default(),
                minion.variant.data().name,
                minion_id.data()
            );
            if minion_id == self.last_attacker {
                name = format!("*{}*", name);
            }
            let stats = minion.stats_print();
            let width = name.len().max(stats.len()) + 2;
            bottom_names.push_str(&format!("{:^width$}", name));
            bottom_stats.push_str(&format!("{:^width$}", stats));
        }

        format!(
            "TOP: {}\n     {}\n\nBOT: {}\n     {}",
            top_names, top_stats, bottom_names, bottom_stats
        )
    }

    pub fn all_minions(&self) -> impl Iterator<Item = &MinionInstance> {
        self.battleground.all_minions().map(|mi_id| self.minion_instances.get(mi_id).unwrap())
    }

    pub fn foreach_minion(&mut self, f: impl Fn(MinionInstanceId, &mut MinionInstance)) {
        for mi_id in self.battleground.all_minions() {
            f(mi_id, self.minion_instances.get_mut(mi_id).unwrap());
        }
    }

    pub fn foreach_minion_of(
        &mut self,
        player_id: PlayerId,
        f: impl Fn(MinionInstanceId, &mut MinionInstance),
    ) {
        for mi_id in self.battleground.player(player_id).board.minions {
            f(mi_id, self.minion_instances.get_mut(mi_id).unwrap());
        }
    }

    pub fn position_minion_at(
        &mut self,
        mi_id: MinionInstanceId,
        position: BoardPosition,
    ) -> Result<(), Error> {
        if position.index >= 7 {
            return Err(Error::FullBoard);
        }
        let player = self.battleground.player_mut(position.player_id);
        let minions = &mut player.board.minions;
        minions
            .try_insert(position.index as usize, mi_id)
            .map_or(Ok(()), |_| Err(Error::FullBoard))?;

        //Update positions
        self.minion_instances.get_mut(mi_id).unwrap().position = Position::BoardPosition(position);
        let index = position.index as usize;
        for &mi_id in minions[(index + 1)..].iter() {
            self.minion_instances.get_mut(mi_id).unwrap().position.unwrap_board_mut().index += 1;
        }
        if position.index < player.next_attack_position {
            player.next_attack_position += 1;
        }
        Ok(())
    }

    pub fn position_minion(
        &mut self,
        mi_id: MinionInstanceId,
        player_id: PlayerId,
    ) -> Result<(), Error> {
        let minions = &mut self.battleground.player_mut(player_id).board.minions;
        let next_index = minions.len() as u8;
        minions.try_push(mi_id).map_or(Ok(()), |_| Err(Error::FullBoard))?;

        //Update positions
        self.minion_instances.get_mut(mi_id).unwrap().position =
            Position::new_on_board(player_id, next_index);
        //No need to update player.next_attack_position as minions positioned last don't impact that.
        Ok(())
    }

    fn remove_minion(&mut self, mi_id: MinionInstanceId) -> Result<(), Error> {
        let minion = self.minion_instances.get(mi_id).unwrap();
        let position = match minion.position {
            Position::BoardPosition(pos) => pos,
            Position::LastPosition(_) => return Err(Error::MinionNotOnBoard),
        };
        let player = self.battleground.player_mut(position.player_id);
        let minions = &mut player.board.minions;
        let index = position.index as usize;
        let removed = minions.remove(index);
        debug_assert_eq!(mi_id, removed);
        self.event_handler_manager.clean_up(mi_id);

        //Update positions
        for &mi_id in minions[index..].iter() {
            self.minion_instances.get_mut(mi_id).unwrap().position.unwrap_board_mut().index -= 1;
        }
        if position.index < player.next_attack_position {
            player.next_attack_position -= 1;
        }

        Ok(())
    }

    fn assert_all_positions_are_correct(&self) {
        for mi_id in self.battleground.all_minions() {
            let position = self.minion_instances.get(mi_id).unwrap().position.unwrap_board();
            if let Some(mi_id_from_pos) = self
                .battleground
                .player(position.player_id)
                .board
                .minions
                .get(position.index as usize)
            {
                debug_assert_eq!(mi_id, *mi_id_from_pos);
            }
        }
    }
}

#[derive(Debug)]
pub struct Replay {
    pub initial_game_snapshot: GameSnapshot,
    pub events: Vec<Event>,
}

impl Replay {
    pub fn new(initial_game_snapshot: GameSnapshot, events: Vec<Event>) -> Self {
        Self {
            initial_game_snapshot,
            events,
        }
    }
}

#[derive(Debug)]
pub struct GameSnapshot {
    pub minions: MinionInstances,
    pub initial_battleground: Battleground,
}

impl GameSnapshot {
    pub fn new(minions: MinionInstances, initial_battleground: Battleground) -> Self {
        Self {
            minions,
            initial_battleground,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BoardSnapshot {
    pub bottom: ArrayVec<[MinionInstance; 7]>,
    pub top: ArrayVec<[MinionInstance; 7]>,
}

impl BoardSnapshot {
    pub fn new(bottom: ArrayVec<[MinionInstance; 7]>, top: ArrayVec<[MinionInstance; 7]>) -> Self {
        Self {
            bottom,
            top,
        }
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Tried to add minion to full board.")]
    FullBoard,
    #[error("Tried to remove minion which is not on the board.")]
    MinionNotOnBoard,
}

#[cfg(test)]
mod test {
    use std::collections::VecDeque;

    use super::*;

    #[test]
    fn deathrattle_summon_token_positions() {
        let mut game = Game::default();

        for _ in 0..7 {
            let icky_imp = game.instantiate_minion(MinionVariant::IckyImp, false);
            game.position_minion(icky_imp, PlayerId::Bottom).unwrap();
        }

        for _ in 0..7 {
            let icky_imp = game.instantiate_minion(MinionVariant::IckyImp, false);
            game.position_minion(icky_imp, PlayerId::Top).unwrap();
        }

        game.initialize();

        game.run();
    }

    //Replays never worked, ignore for the moment
    #[ignore]
    #[test]
    fn replay_simulated_game() {
        let mut game = Game::default();
        let mut rng = thread_rng();
        for _ in 0..7 {
            let minion = game.instantiate_minion(MinionVariant::random(&mut rng), false);
            game.position_minion(minion, PlayerId::Bottom).unwrap();
        }
        for _ in 0..7 {
            let minion = game.instantiate_minion(MinionVariant::random(&mut rng), false);
            game.position_minion(minion, PlayerId::Top).unwrap();
        }
        game.initialize();

        let mut game_board_snapshots = VecDeque::new();
        let replay = {
            let game_snapshot = game.game_snapshot();
            let mut events = Vec::new();
            loop {
                game_board_snapshots.push_back(game.board_snapshot());

                let current_event = game.step();
                if let Event::End(_) = current_event {
                    events.push(current_event);
                    break Replay::new(game_snapshot, events);
                }
                events.push(current_event);
            }
        };

        let mut game_replayer = Game::from_replay(replay);

        loop {
            assert_eq!(game_replayer.board_snapshot(), game_board_snapshots.pop_front().unwrap());

            match game_replayer.replay_step() {
                Event::End(_) => break,
                Event::Invalid => panic!("Incomplete replay"),
                _ => (),
            }
        }
    }
}
