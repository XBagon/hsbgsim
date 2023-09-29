use hsbgsim::*;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;
use std::env;

pub fn main() {
    let mut args = env::args().skip(1);
    let seed: u64 = args.next().unwrap().parse().unwrap();

    let mut game = Game::with_seed(seed);

    let mut rng = Xoshiro256PlusPlus::seed_from_u64(!seed);

    for _ in 0..7 {
        let minion = game.instantiate_minion(MinionVariant::random(&mut rng));
        game.position_minion(minion, PlayerId::Bottom).unwrap();

        let minion = game.instantiate_minion(MinionVariant::random(&mut rng));
        game.position_minion(minion, PlayerId::Top).unwrap();
    }

    game.initialize();

    game.run_and_print();
}
