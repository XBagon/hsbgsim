use std::{env, str::FromStr};

use hsbgsim::*;

fn main() {
    let mut args = env::args().skip(1);
    let bottom_minion_name = args.next().unwrap();
    let top_minion_name = args.next();

    fn parse_name(name: &str) -> (MinionVariant, bool) {
        if let Some(name) = name.strip_prefix("G-") {
            (MinionVariant::from_str(&name).unwrap(), true)
        } else {
            (MinionVariant::from_str(&name).unwrap(), false)
        }
    }

    let mut bottom_minion = parse_name(&bottom_minion_name);
    let mut top_minion = top_minion_name.map(|name| parse_name(&name)).unwrap_or(bottom_minion);

    let mut game = Game::default();

    for _ in 0..7 {
        let minion = game.instantiate_minion(bottom_minion.0, bottom_minion.1);
        game.position_minion(minion, PlayerId::Bottom).unwrap();

        let minion = game.instantiate_minion(top_minion.0, top_minion.1);
        game.position_minion(minion, PlayerId::Top).unwrap();
    }

    game.initialize();

    game.run_and_print();
}
