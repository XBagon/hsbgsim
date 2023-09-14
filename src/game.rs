use core::panic;
use std::{fmt::format, os::windows::prelude::OsStringExt};

use crate::{
    events::{
        AssociatedEventHandler, Attack, Death, DeathCheck, End, Event, EventHandler,
        EventHandlerManager, Events, ProposeAttack, TakeDamage,
    },
    minions::MinionInstanceId,
    minions::MinionVariant,
    minions::Position,
    player::PlayerId,
    Battleground, MinionInstance,
};
use bitmaps::Bitmap;
use rand::seq::SliceRandom;
use slotmap::{Key, SlotMap};
use thiserror::Error;
use tinyvec::ArrayVec;

#[derive(Default)]
pub struct Game {
    pub battleground: Battleground,
    pub minion_instances: SlotMap<MinionInstanceId, MinionInstance>,
    events: Events,
    event_handler_manager: EventHandlerManager,
    just_checked_deaths: bool,
}

struct GameBuilder {}

impl GameBuilder {
    pub fn new() -> Self {
        Self {}
    }
}

impl Game {
    pub fn instantiate_minion(&mut self, variant: MinionVariant) -> MinionInstanceId {
        let minion_instance = variant.into_instance();
        let mi_id = self.minion_instances.insert(minion_instance);
        self.event_handler_manager.append_event_handler(mi_id, &variant.event_handler());
        mi_id
    }

    pub fn step(&mut self) -> Event {
        let next_event = self.events.next();
        let next_event = if let Some(next_event) = next_event {
            next_event
        } else {
            //Check if game is over
            let bottom_empty = self.battleground.player(PlayerId::Bottom).board.minions.is_empty();
            let top_empty = self.battleground.player(PlayerId::Top).board.minions.is_empty();
            match (bottom_empty, top_empty) {
                (true, true) => End::Draw.into(),
                (true, false) => End::TopWin.into(),
                (false, true) => End::BottomWin.into(),
                (false, false) => ProposeAttack::new(
                    self.random_target(PlayerId::Bottom),
                    self.random_target(PlayerId::Top),
                )
                .into(),
            }
        };

        match &next_event {
            Event::Invalid => todo!(),
            Event::End(end) => {}
            &Event::ProposeAttack(propose_attack) => {
                self.push_event(Event::Attack(propose_attack.into()));
                for i in 0..self.event_handler_manager.propose_attack.len() {
                    let AssociatedEventHandler {minion: mi_id, handler} = self.event_handler_manager.propose_attack[i];
                    handler(mi_id, propose_attack, self)
                }
            }
            &Event::Attack(attack) => {
                self.push_event(Event::AfterAttack(attack.into()));
                let [attacker, defender] = self
                    .minion_instances
                    .get_disjoint_mut([attack.attacker, attack.defender])
                    .unwrap();
                let mut events = ArrayVec::<[Event; 2]>::new();
                if defender.attack > 0 {
                    if attacker.abilities.shield() {
                        attacker.abilities.set_shield(false);
                    } else {
                        let event = TakeDamage::new(attack.attacker, defender.attack).into();
                        events.push(event);
                    }
                }
                if attacker.attack > 0 {
                    if defender.abilities.shield() {
                        defender.abilities.set_shield(false);
                    } else {
                        let event = TakeDamage::new(attack.defender, attacker.attack).into();
                        events.push(event);
                    }
                }
                events.into_iter().for_each(|event| self.push_event(event));
                //for i in 0..self.event_handler_manager.attack.len() {
                //    let AssociatedEventHandler {minion: mi_id, handler} = self.event_handler_manager.attack[i];
                //    handler(mi_id, attack, self)
                //}
            }
            &Event::AfterAttack(attack) => {
                let [attacker, defender] = self
                    .minion_instances
                    .get_disjoint_mut([attack.attacker, attack.defender])
                    .unwrap();
            }
            &Event::DeathCheck(_death_check) => {
                let event_count = self.events.len();
                for mi_id in self.battleground.all_minions() {
                    let minion = self.minion_instances.get(mi_id).unwrap();
                    if minion.health <= 0 {
                        self.events.push(Death::new(mi_id).into());
                    }
                }
            }
            &Event::Death(death) => {
                self.remove_minion(death.minion).unwrap();
            }
            &Event::StatBuff(stat_buff) => {
                let target = self.minion_instances.get_mut(stat_buff.target).unwrap();
                target.attack += stat_buff.attack;
                target.health += stat_buff.health;
            }
            &Event::TakeDamage(take_damage) => {
                self.minion_instances.get_mut(take_damage.target).unwrap().health -= take_damage.amount;
                for i in 0..self.event_handler_manager.take_damage.len() {
                    let AssociatedEventHandler {minion: mi_id, handler} = self.event_handler_manager.take_damage[i];
                    handler(mi_id, take_damage, self)
                }
            }
            //Event::TauntMinionAdded(_) => todo!(),
            //Event::TauntMinionRemoved(_) => todo!(),
            //Event::StealthAdded(_) => todo!(),
            //Event::StealthRemoved(_) => todo!(),
            //Event::MinionAdded(_) => todo!(),
            //Event::MinionRemoved(_) => todo!(),
        }

        next_event
    }

    pub fn push_event(&mut self, event: Event) {
        if self.events.len() == 0 {
            //OUTER PHASE: Check Deaths here
            self.events.push(DeathCheck.into());
        }
        self.events.push(event);
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
    //fn process_next_event(&mut self) {
    //    let next_event = self.events.next().unwrap_or(Event::Invalid);
    //    match next_event {
    //        Event::Attack {
    //            attacker,
    //            defender,
    //        } => todo!(),
    //        Event::TauntMinionAdded(id) => {
    //            let minion = &self.minion_instances[id];
    //            let another_taunt = self.has_taunt_up(minion.position.player_id);
    //            let board = &mut self.battleground.player_mut(minion.position.player_id).board;
    //            if another_taunt {
    //                board.valid_target_map.set(minion.position.index as usize, true);
    //            } else {
    //                board.valid_target_map = Bitmap::new();
    //                board.valid_target_map.set(minion.position.index as usize, true);
    //            }
    //        }
    //        Event::TauntMinionRemoved(id) => {
    //            let minion = &self.minion_instances[id];
    //            let board = &mut self.battleground.player_mut(minion.position.player_id).board;
    //            board.valid_target_map.set(minion.position.index as usize, false);
    //            if board.valid_target_map.is_empty() {
    //                //no taunts anymore
    //                self.recalculate_valid_target_map(minion.position.player_id);
    //            }
    //        }
    //        Event::StealthAdded(id) => {
    //            let minion = &self.minion_instances[id];
    //            let board = &mut self.battleground.player_mut(minion.position.player_id).board;
    //            board.valid_target_map.set(minion.position.index as usize, false);
    //            if minion.abilities.taunt() {
    //                self.events.push(Event::TauntMinionRemoved(id));
    //            }
    //        }
    //        Event::StealthRemoved(id) => {
    //            let minion = &self.minion_instances[id];
    //            if minion.abilities.taunt() {
    //                self.events.push(Event::TauntMinionAdded(id));
    //            } else if !self.has_taunt_up(minion.position.player_id) {
    //                let board = &mut self.battleground.player_mut(minion.position.player_id).board;
    //                board.valid_target_map.set(minion.position.index as usize, true);
    //            }
    //        }
    //        Event::MinionAdded(id) => {
    //            todo!()
    //        }
    //        Event::MinionRemoved(id) => {
    //            let minion = &self.minion_instances[id];
    //            let board = &mut self.battleground.player_mut(minion.position.player_id).board;
    //            let bitmap = board.valid_target_map.into_value();
    //            board.valid_target_map =
    //                Bitmap::from_value((bitmap & (1 << minion.position.index - 1)) | bitmap >> 1);
    //            todo!()
    //        }
    //        Event::Invalid => panic!("Invalid Event"),
    //    }
    //}

    //pub fn valid_targets(&self, player_id: PlayerId) -> Vec<MinionInstanceId> {
    //    let board = &self.battleground.player(player_id).board;
    //    board.valid_target_map.into_iter().map(|i| board.minions[i]).collect()
    //}
    //
    //pub fn has_taunt_up(&self, player_id: PlayerId) -> bool {
    //    let board = &self.battleground.player(player_id).board;
    //    board
    //        .valid_target_map
    //        .first_index()
    //        .map(|i| self.minion_instances[board.minions[i]].abilities.taunt())
    //        .unwrap_or(false)
    //}

    //pub fn recalculate_valid_target_maps(&mut self) {
    //    for player in self.battleground.players_mut() {
    //        let board = &mut player.board;
    //        for (i, &minion_id) in board.minions.iter().enumerate() {
    //            let minion = &self.minion_instances[minion_id];
    //            board.valid_target_map.set(i, !minion.abilities.stealth());
    //        }
    //    }
    //}

    //pub fn recalculate_valid_target_map(&mut self, player_id: PlayerId) {
    //    let board = &mut self.battleground.player_mut(player_id).board;
    //    for (i, &minion_id) in board.minions.iter().enumerate() {
    //        let minion = &self.minion_instances[minion_id];
    //        board.valid_target_map.set(i, !minion.abilities.stealth());
    //    }
    //}

    pub fn random_target(&self, player_id: PlayerId) -> MinionInstanceId {
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
        *valid_targets.choose(&mut rand::thread_rng()).unwrap()
    }

    pub fn game_print(&self) -> String {
        let mut top_names = String::new();
        let mut top_stats = String::new();
        for minion_id in self.battleground.player(PlayerId::Top).board.minions {
            let minion = &self.minion_instances[minion_id];
            let mut name = minion.variant.data().name;
            name.push_str(&format!("|{:?}", minion_id.data()));
            let mut stats = minion.stats_print();
            let width = name.len().max(stats.len()) + 2;
            top_names.push_str(&format!("{:^width$}", name));
            top_stats.push_str(&format!("{:^width$}", stats));
        }

        let mut bottom_names = String::new();
        let mut bottom_stats = String::new();
        for minion_id in self.battleground.player(PlayerId::Bottom).board.minions {
            let minion = &self.minion_instances[minion_id];
            let mut name = minion.variant.data().name;
            name.push_str(&format!("|{:?}", minion_id.data()));
            let mut stats = minion.stats_print();
            let width = name.len().max(stats.len()) + 2;
            bottom_names.push_str(&format!("{:^width$}", name));
            bottom_stats.push_str(&format!("{:^width$}", stats));
        }

        format!(
            "TOP: {}\n     {}\n\nBOT: {}\n     {}",
            top_names, top_stats, bottom_names, bottom_stats
        )
    }

    pub fn position_minion_at(
        &mut self,
        mi_id: MinionInstanceId,
        position: Position,
    ) -> Result<(), Error> {
        let res = self
            .battleground
            .player_mut(position.player_id)
            .board
            .minions
            .try_insert(position.index as usize, mi_id)
            .map_or(Ok(()), |_| Err(Error::FullBoard));
        self.minion_instances.get_mut(mi_id).unwrap().position = Some(position);
        //TODO: modify position of shifted minions accordingly
        res
    }

    pub fn position_minion(
        &mut self,
        mi_id: MinionInstanceId,
        player_id: PlayerId,
    ) -> Result<(), Error> {
        let mut minions = &mut self.battleground.player_mut(player_id).board.minions;
        let next_index = minions.len() as u8;
        minions.try_push(mi_id).map_or(Ok(()), |_| Err(Error::FullBoard))?;
        self.minion_instances.get_mut(mi_id).unwrap().position =
            Some(Position::new(player_id, next_index));
        Ok(())
    }

    fn remove_minion(&mut self, mi_id: MinionInstanceId) -> Result<(), Error> {
        let minion = self.minion_instances.get(mi_id).unwrap();
        let position = minion.position.ok_or(Error::MinionNotOnBoard)?;
        let minions = &mut self.battleground.player_mut(position.player_id).board.minions;
        let index = position.index as usize;
        let removed = minions.remove(index);
        for &mi_id in minions[index..].iter() {
            self.minion_instances.get_mut(mi_id).unwrap().position.as_mut().unwrap().index -= 1;
        }
        debug_assert_eq!(mi_id, removed);

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Tried to add minion to full board.")]
    FullBoard,
    #[error("Tried to remove minion which is not on the board.")]
    MinionNotOnBoard,
}
