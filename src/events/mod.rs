use crate::{game::Game, minions::MinionInstanceId};
use paste::paste;

mod after_attack;
mod attack;
pub mod common_effects;
mod death;
mod death_check;
mod end;
mod propose_attack;
mod stat_buff;
mod take_damage;

pub use after_attack::AfterAttack;
pub use attack::Attack;
pub use death::Death;
pub use death_check::DeathCheck;
pub use end::End;
pub use propose_attack::ProposeAttack;
pub use stat_buff::StatBuff;
pub use take_damage::TakeDamage;

#[derive(Default)]
pub struct Events {
    queue: Vec<Event>,
}

pub struct AssociatedEventHandler<H> {
    pub minion: MinionInstanceId,
    pub handler: H,
}

impl<H> AssociatedEventHandler<H> {
    pub fn new(minion: MinionInstanceId, handler: H) -> Self {
        Self {
            minion,
            handler,
        }
    }
}

macro_rules! event_variants {
    ($($vars:ident),*) => {
        #[derive(Debug, Default)]
        pub enum Event {
            #[default]
            Invalid,
            $($vars($vars)),*
        }

        $(impl From<$vars> for Event {
            fn from(value: $vars) -> Self {
                Event::$vars(value)
            }
        })*

        paste!{
            #[derive(Default)]
            pub struct EventHandlerManager {
                $(pub [<$vars:snake>]: Vec<AssociatedEventHandler<fn(MinionInstanceId, $vars, &mut Game)>>),*
            }

            impl EventHandlerManager {
               pub fn append_event_handler(&mut self, mi_id: MinionInstanceId, other: &EventHandler) {
                $(self.[<$vars:snake>].extend(other.[<$vars:snake>].map(|handler| AssociatedEventHandler::new(mi_id, handler))));*
               }
            }

            #[derive(Default)]
            pub struct EventHandler {
                $(pub [<$vars:snake>]: Option<fn(MinionInstanceId, $vars, &mut Game)>),*
            }

        }
    };
}

event_variants!(Attack, ProposeAttack, AfterAttack, DeathCheck, Death, End, StatBuff, TakeDamage);

impl Events {
    pub fn push(&mut self, event: Event) {
        self.queue.push(event);
    }

    pub fn next(&mut self) -> Option<Event> {
        self.queue.pop()
    }

    pub(crate) fn len(&self) -> usize {
        self.queue.len()
    }
}
