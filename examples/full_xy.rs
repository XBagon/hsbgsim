use std::{env, str::FromStr};

use hsbgsim::*;

fn main() {
    let mut args = env::args().skip(1);
    let bottom_minion_name = args.next().unwrap();
    let top_minion_name = args.next();

    let mut game = Game::default();

    let bottom_minion = MinionVariant::from_str(&bottom_minion_name).unwrap();
    let top_minion = top_minion_name
        .map(|name| MinionVariant::from_str(&name).unwrap())
        .unwrap_or(bottom_minion);

    for _ in 0..7 {
        let annoy_o_tron = game.instantiate_minion(bottom_minion, false);
        game.position_minion(annoy_o_tron, PlayerId::Bottom).unwrap();

        let annoy_o_tron = game.instantiate_minion(top_minion, false);
        game.position_minion(annoy_o_tron, PlayerId::Top).unwrap();
    }

    game.initialize();

    game.run_and_print();
}
