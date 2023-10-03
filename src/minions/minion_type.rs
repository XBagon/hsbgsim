use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MinionType {
    #[default]
    Invalid,
    Beast,
    Demon,
    Dragon,
    Elemental,
    Mech,
    Murloc,
    Naga,
    Pirate,
    Quilboar,
    Totem,
    Undead,
    All,
}
