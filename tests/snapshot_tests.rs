use hsbgsim::*;
use insta::assert_yaml_snapshot;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;
use test_case::test_matrix;

#[test_matrix(
    0..100
)]
pub fn random_board_snapshots(seed: u64) {
    let mut game = Game::with_seed(seed);

    let mut rng = Xoshiro256PlusPlus::seed_from_u64(!seed);

    for _ in 0..7 {
        let minion = game.instantiate_minion(MinionVariant::Bonker);
        game.position_minion(minion, PlayerId::Bottom).unwrap();

        let minion = game.instantiate_minion(MinionVariant::Bonker);
        game.position_minion(minion, PlayerId::Top).unwrap();
    }

    game.initialize();

    let recording = game.run_and_record_events();

    assert_yaml_snapshot!(recording);
}

#[test_matrix(
    0..100
)]
pub fn random_board(seed: u64) {
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
