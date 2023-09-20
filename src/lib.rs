mod battleground;
mod board;
mod events;
mod game;
mod hand;
mod hero;
mod minions;
mod player;

pub use battleground::Battleground;
pub use board::Board;
pub use events::{End, Event};
pub use game::Game;
pub use hand::Hand;
pub use hero::Hero;
pub use minions::{MinionInstance, MinionInstanceId, MinionVariant};
pub use player::{Player, PlayerId};
//https://bgknowhow.com/bgjson/
//https://bgknowhow.com/bgjson/output/bg_entities_active.json

//TODO: set up CI
