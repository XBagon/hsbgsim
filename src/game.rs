use crate::{
    events::{
        AssociatedEventHandler, Death, DeathCheck, End, Event, EventHandlerManager, Events,
        ProposeAttack, TakeDamage,
    },
    minions::MinionInstanceId,
    minions::MinionVariant,
    minions::Position,
    player::PlayerId,
    Battleground, MinionInstance,
};

use rand::prelude::*;
use rand_xoshiro::Xoshiro256PlusPlus;
use slotmap::{Key, SlotMap};
use thiserror::Error;
use tinyvec::ArrayVec;

pub struct Game {
    pub battleground: Battleground,
    pub minion_instances: SlotMap<MinionInstanceId, MinionInstance>,
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
            attacking_player: Default::default(),
            last_attacker: Default::default(),
            additional_attacks: Default::default(),
            events: Default::default(),
            event_handler_manager: Default::default(),
            rng: Xoshiro256PlusPlus::seed_from_u64(seed),
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
    }

    pub fn instantiate_minion(&mut self, variant: MinionVariant) -> MinionInstanceId {
        let minion_instance = variant.into_instance();
        let mi_id = self.minion_instances.insert(minion_instance);
        self.event_handler_manager.append_event_handler(mi_id, &variant.event_handler());
        mi_id
    }

    pub fn step(&mut self) -> Event {
        #[cfg(any(debug_assertions, test))]
        self.assert_all_positions_are_correct();

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
                (false, false) => {
                    let attacking_player_id = self.attacking_player.unwrap();
                    let next_player_id = attacking_player_id.oppsite();

                    let attack_info = if self.additional_attacks > 0 && {
                        self.additional_attacks -= 1;
                        let minion = self.minion_instances.get(self.last_attacker).unwrap();
                        minion.position.is_some() && minion.attack > 0
                    } {
                        //Last minion attacks again
                        Some((self.last_attacker, attacking_player_id))
                    } else {
                        if let Some(attacker) = self.determine_next_attacker(attacking_player_id) {
                            Some((attacker, next_player_id))
                        } else {
                            //Switch attacker
                            if let Some(attacker) = self.determine_next_attacker(next_player_id) {
                                Some((attacker, attacking_player_id))
                            } else {
                                None
                            }
                        }
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

        match &next_event {
            Event::Invalid => unreachable!(),
            Event::End(_end) => {}
            &Event::ProposeAttack(propose_attack) => {
                self.push_event(Event::Attack(propose_attack.into()));
                if propose_attack.is_outer_phase {
                    self.push_event(DeathCheck.into());
                }
                for i in 0..self.event_handler_manager.propose_attack.len() {
                    let AssociatedEventHandler {
                        minion: mi_id,
                        handler,
                    } = self.event_handler_manager.propose_attack[i];
                    handler(mi_id, propose_attack, self)
                }
            }
            &Event::Attack(attack) => {
                self.push_event(Event::AfterAttack(attack.into()));
                if attack.is_outer_phase {
                    self.push_event(DeathCheck.into());
                }
                let [attacker, defender] = self
                    .minion_instances
                    .get_disjoint_mut([attack.attacker, attack.defender])
                    .unwrap();
                debug_assert_ne!(
                    attacker.position.unwrap().player_id,
                    defender.position.unwrap().player_id
                );
                let mut events = ArrayVec::<[Event; 2]>::new();
                if defender.attack > 0 {
                    if attacker.abilities.shield() {
                        attacker.abilities.set_shield(false);
                    } else {
                        let event =
                            TakeDamage::new(attack.attacker, defender.attack, attack.defender)
                                .into();
                        events.push(event);
                    }
                    if attacker.abilities.venomous() {
                        attacker.abilities.set_venomous(false);
                        defender.pending_destroy = true;
                    }
                }
                if attacker.attack > 0 {
                    if defender.abilities.shield() {
                        defender.abilities.set_shield(false);
                    } else {
                        let event =
                            TakeDamage::new(attack.defender, attacker.attack, attack.attacker)
                                .into();
                        events.push(event);
                    }
                    if defender.abilities.venomous() {
                        defender.abilities.set_venomous(false);
                        attacker.pending_destroy = true;
                    }
                }
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
                    self.push_event(DeathCheck.into());
                }
            }
            &Event::DeathCheck(_death_check) => {
                let _event_count = self.events.len();
                for mi_id in self.battleground.all_minions() {
                    let minion = self.minion_instances.get(mi_id).unwrap();
                    if minion.health <= 0 || minion.pending_destroy {
                        self.events.push(Death::new(mi_id).into());
                    }
                }
            }
            &Event::Death(death) => {
                self.remove_minion(death.minion).unwrap();
                for i in 0..self.event_handler_manager.death.len() {
                    let AssociatedEventHandler {
                        minion: mi_id,
                        handler,
                    } = self.event_handler_manager.death[i];
                    handler(mi_id, death, self)
                }
            }
            &Event::StatBuff(stat_buff) => {
                let target = self.minion_instances.get_mut(stat_buff.target).unwrap();
                target.attack += stat_buff.attack;
                target.health += stat_buff.health;
            }
            &Event::TakeDamage(take_damage) => {
                self.minion_instances.get_mut(take_damage.target).unwrap().health -=
                    take_damage.amount;
                for i in 0..self.event_handler_manager.take_damage.len() {
                    let AssociatedEventHandler {
                        minion: mi_id,
                        handler,
                    } = self.event_handler_manager.take_damage[i];
                    handler(mi_id, take_damage, self)
                }
            }
            Event::Summon(summon) => {
                _ = self.position_minion_at(summon.minion, summon.position);
            }
        }

        next_event
    }

    pub fn push_event(&mut self, event: Event) {
        //if self.events.len() == 0 {
        //    //OUTER PHASE: Check Deaths here
        //    self.events.push(DeathCheck.into());
        //}
        self.events.push(event);
    }

    pub fn run(&mut self) -> End {
        loop {
            let current_event = self.step();

            if let Event::End(result) = current_event {
                return result;
            }
        }
    }

    pub fn run_and_record_events(&mut self) -> Vec<Event> {
        let mut recording = Vec::new();

        loop {
            let current_event = self.step();

            if let Event::End(_) = current_event {
                recording.push(current_event);
                return recording;
            }

            recording.push(current_event);
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
            if next_minion.attack > 0 {
                attacking_player.next_attack_position = i as u8 + 1;
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
            let mut name = minion.variant.data().name;
            name.push_str(&format!("|{:?}", minion_id.data()));
            let stats = minion.stats_print();
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

    pub fn position_minion_at(
        &mut self,
        mi_id: MinionInstanceId,
        position: Position,
    ) -> Result<(), Error> {
        if position.index >= 7 {
            return Err(Error::FullBoard);
        }
        let player = self.battleground.player_mut(position.player_id);
        let minions = &mut player.board.minions;
        minions
            .try_insert(position.index as usize, mi_id)
            .map_or(Ok(()), |_| Err(Error::FullBoard))?;
        self.minion_instances.get_mut(mi_id).unwrap().position = Some(position);
        let index = position.index as usize;
        for &mi_id in minions[(index + 1)..].iter() {
            self.minion_instances.get_mut(mi_id).unwrap().position.as_mut().unwrap().index += 1;
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
        self.minion_instances.get_mut(mi_id).unwrap().position =
            Some(Position::new(player_id, next_index));
        //No need to update player.next_attack_position as minions positioned last don't impact that.
        Ok(())
    }

    fn remove_minion(&mut self, mi_id: MinionInstanceId) -> Result<(), Error> {
        let minion = self.minion_instances.get(mi_id).unwrap();
        let position = minion.position.ok_or(Error::MinionNotOnBoard)?;
        let player = self.battleground.player_mut(position.player_id);
        let minions = &mut player.board.minions;
        let index = position.index as usize;
        let removed = minions.remove(index);
        debug_assert_eq!(mi_id, removed);
        for &mi_id in minions[index..].iter() {
            self.minion_instances.get_mut(mi_id).unwrap().position.as_mut().unwrap().index -= 1;
        }
        if position.index < player.next_attack_position {
            player.next_attack_position -= 1;
        }

        Ok(())
    }

    fn assert_all_positions_are_correct(&self) {
        for mi_id in self.battleground.all_minions() {
            let position = self.minion_instances.get(mi_id).unwrap().position.unwrap();
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

#[derive(Error, Debug)]
pub enum Error {
    #[error("Tried to add minion to full board.")]
    FullBoard,
    #[error("Tried to remove minion which is not on the board.")]
    MinionNotOnBoard,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deathrattle_summon_token_positions() {
        let mut game = Game::default();

        for _ in 0..7 {
            let icky_imp = game.instantiate_minion(MinionVariant::IckyImp);
            game.position_minion(icky_imp, PlayerId::Bottom).unwrap();
        }

        for _ in 0..7 {
            let icky_imp = game.instantiate_minion(MinionVariant::IckyImp);
            game.position_minion(icky_imp, PlayerId::Top).unwrap();
        }

        game.initialize();

        game.run();
    }
}
