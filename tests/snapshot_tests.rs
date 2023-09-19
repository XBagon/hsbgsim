use hsbgsim::*;
use insta::assert_yaml_snapshot;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;
use std::collections::BTreeMap;
use test_case::test_matrix;

#[test_matrix(
    0..100
)]
pub fn random_board_snapshot(seed: u64) {
    let mut game = Game::with_seed(seed);

    let mut rng = Xoshiro256PlusPlus::seed_from_u64(!seed);

    for _ in 0..7 {
        let minion = game.instantiate_minion(MinionVariant::random(&mut rng));
        game.position_minion(minion, PlayerId::Bottom).unwrap();

        let minion = game.instantiate_minion(MinionVariant::random(&mut rng));
        game.position_minion(minion, PlayerId::Top).unwrap();
    }

    game.initialize();

    let recording = game.run_and_record_events();
    let mut minions: BTreeMap<_, _> =
        game.minion_instances.iter().map(|(mi_id, minion)| (mi_id, minion.variant)).collect();

    //TODO: better replay format
    assert_yaml_snapshot!(format!("random_board_snapshot_{}", seed), (minions, recording));
}
