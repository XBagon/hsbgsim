use crate::{game::Game, minions::MinionInstanceId};
use paste::paste;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

mod activate_effect;
mod after_attack;
mod attack;
pub mod common_effects;
mod death;
mod death_check;
mod end;
mod propose_attack;
mod remove;
mod stat_buff;
mod summon;
mod take_damage;
//TODO: AfterEffect;

pub use activate_effect::ActivateEffect as RawActivateEffect;
pub use after_attack::AfterAttack;
pub use attack::Attack;
pub use death::Death;
pub use death_check::DeathCheck;
pub use end::End;
pub use propose_attack::ProposeAttack;
pub use remove::Remove;
pub use stat_buff::StatBuff;
pub use summon::Summon;
pub use take_damage::TakeDamage;

type ActivateEffect = Box<activate_effect::ActivateEffect>;

pub trait EventTrait: Clone {
    type Handler: Fn(MinionInstanceId, Self, &mut Game) + Debug + Clone;
}

#[derive(Default)]
pub struct Events {
    queue: Vec<Event>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AssociatedEventHandler<Ev: EventTrait> {
    pub minion: MinionInstanceId,
    //TODO: manual serialization from `Game` could lookup additional information about handler in a hashmap.
    #[serde(skip)]
    pub handler: Option<Ev::Handler>,
}

impl<Ev: EventTrait> AssociatedEventHandler<Ev> {
    pub fn new(minion: MinionInstanceId, handler: Ev::Handler) -> Self {
        Self {
            minion,
            handler: Some(handler),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventHandling<Ev: EventTrait> {
    pub event: Ev,
    pub ass_handler: AssociatedEventHandler<Ev>,
}

impl<Ev: EventTrait> EventHandling<Ev> {
    pub fn new(event: Ev, ass_handler: AssociatedEventHandler<Ev>) -> Self {
        Self {
            event,
            ass_handler,
        }
    }

    pub fn handle(&self, game: &mut Game) {
        let AssociatedEventHandler {
            minion: mi_id,
            handler,
        } = &self.ass_handler;
        //Clone the Option or the handler? Does it make a difference?
        (handler.clone().unwrap())(*mi_id, self.event.clone(), game);
    }
}

macro_rules! event_variants {
    ($($vars:ident),*) => {
        paste!{
            $(impl EventTrait for $vars {
                type Handler = fn(MinionInstanceId, Self, &mut Game);
            })*

            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub enum GenericEventHandling {
                $($vars(EventHandling::<$vars>),)*
            }

            impl GenericEventHandling {
                pub fn handle(&self, game: &mut Game) {
                    match self {
                        $(Self::$vars(event_handling) => event_handling.handle(game),)*
                    }
                }
            }

            $(impl From<EventHandling::<$vars>> for GenericEventHandling {
                fn from(value: EventHandling::<$vars>) -> Self {
                    Self::$vars(value)
                }
            })*

            #[derive(Debug, Default, Serialize, Deserialize)]
            pub enum Event {
                #[default]
                Invalid,
                $($vars($vars),)*
            }

            $(impl From<$vars> for Event {
                fn from(value: $vars) -> Self {
                    Event::$vars(value)
                }
            })*

            #[derive(Default)]
            pub struct EventHandlerManager {
                $(pub [<$vars:snake>]: Vec<AssociatedEventHandler<$vars>>),*
            }

            impl EventHandlerManager {
               pub fn append_event_handler(&mut self, mi_id: MinionInstanceId, other: &EventHandlers) {
                $(self.[<$vars:snake>].extend(other.[<$vars:snake>].map(|handler| AssociatedEventHandler::new(mi_id, handler)));)*
               }

               pub fn clean_up(&mut self, mi_id: MinionInstanceId) {
                $(self.[<$vars:snake>].retain(|ass_ev_handler| ass_ev_handler.minion != mi_id);)*
               }
            }

            #[derive(Default)]
            pub struct EventHandlers {
                $(pub [<$vars:snake>]: Option<fn(MinionInstanceId, $vars, &mut Game)>),*
            }

        }
    };
}

event_variants!(
    Attack,
    ProposeAttack,
    AfterAttack,
    DeathCheck,
    Death,
    Remove,
    End,
    StatBuff,
    TakeDamage,
    Summon,
    ActivateEffect
);

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
