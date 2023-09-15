use hsbgsim::*;

pub fn main() {
    let mut game = Game::default();

    for _ in 0..7 {
        let minion = game.instantiate_minion(MinionVariant::random());
        game.position_minion(minion, PlayerId::Bottom).unwrap();

        let minion = game.instantiate_minion(MinionVariant::random());
        game.position_minion(minion, PlayerId::Top).unwrap();
    }

    game.initialize();

    game.run_and_print();
}
