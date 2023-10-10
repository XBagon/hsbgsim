use std::{env, str::FromStr};

use hsbgsim::*;

fn main() {
    let args = env::args().skip(1);

    fn parse_name(name: &str) -> (MinionVariant, bool) {
        if let Some(name) = name.strip_prefix("G-") {
            (MinionVariant::from_str(&name).unwrap(), true)
        } else {
            (MinionVariant::from_str(&name).unwrap(), false)
        }
    }

    let mut bottom_minions = Vec::new();
    let mut top_minions = Vec::new();

    let mut current = &mut bottom_minions;
    for arg in args {
        if arg == "--" {
            current = &mut top_minions;
        } else {
            current.push(parse_name(&arg));
        }
    }

    let mut game = Game::default();

    for minion in bottom_minions {
        let minion = game.instantiate_minion(minion.0, minion.1);
        game.position_minion(minion, PlayerId::Bottom).unwrap();
    }

    for minion in top_minions {
        let minion = game.instantiate_minion(minion.0, minion.1);
        game.position_minion(minion, PlayerId::Top).unwrap();
    }

    game.initialize();

    game.run_and_print();
}
