use hsbgsim::*;
use insta::assert_yaml_snapshot;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;
use std::collections::{BTreeMap, BTreeSet};
use test_case::test_matrix;

//Has to be the same range as in the coverage test
#[test_matrix(
    0..300
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
    let minions: BTreeMap<_, _> =
        game.minion_instances.iter().map(|(mi_id, minion)| (mi_id, minion.variant)).collect();

    //TODO: better replay format
    assert_yaml_snapshot!(format!("random_board_snapshot_{}", seed), (minions, recording));
}

#[test]
fn coverage() {
    let mut variants_left = BTreeSet::from_iter(MinionVariant::ALL.iter());
    let variant_count = variants_left.len();

    //Has to be the same range as on the test_matrix
    for seed in 0..300 {
        let mut game = Game::with_seed(seed);

        let mut rng = Xoshiro256PlusPlus::seed_from_u64(!seed);

        for _ in 0..7 {
            let minion = game.instantiate_minion(MinionVariant::random(&mut rng));
            game.position_minion(minion, PlayerId::Bottom).unwrap();

            let minion = game.instantiate_minion(MinionVariant::random(&mut rng));
            game.position_minion(minion, PlayerId::Top).unwrap();
        }

        for (_mi_id, minion) in game.minion_instances {
            variants_left.remove(&minion.variant);
        }
    }

    assert!(variants_left.len() == 0, "{}/{} not covered.", variants_left.len(), variant_count)
}

//TODO: test implemented minions more targeted to reduce the unnecessary hight amount of tests in this state
