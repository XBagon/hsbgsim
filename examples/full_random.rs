use hsbgsim::*;
use rand::Rng;

pub fn main() {
    let mut game = Game::default();

    let mut rng = rand::thread_rng();

    for _ in 0..7 {
        let minion = game.instantiate_minion(MinionVariant::random(&mut rng), rng.gen());
        game.position_minion(minion, PlayerId::Bottom).unwrap();

        let minion = game.instantiate_minion(MinionVariant::random(&mut rng), rng.gen());
        game.position_minion(minion, PlayerId::Top).unwrap();
    }

    game.initialize();

    game.run_and_print();
}
