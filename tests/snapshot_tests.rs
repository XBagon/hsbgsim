use hsbgsim::*;
use insta::assert_yaml_snapshot;
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;
use serde::{Deserialize, Serialize};
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
        let minion = game.instantiate_minion(MinionVariant::random(&mut rng), rng.gen());
        game.position_minion(minion, PlayerId::Bottom).unwrap();

        let minion = game.instantiate_minion(MinionVariant::random(&mut rng), rng.gen());
        game.position_minion(minion, PlayerId::Top).unwrap();
    }

    game.initialize();

    let recording = game.run_and_record();
    let minions: BTreeMap<_, _> = game
        .minion_instances
        .iter()
        .map(|(mi_id, minion)| {
            (
                mi_id,
                format!(
                    "{}{}",
                    minion.golden.then_some(String::from("Golden ")).unwrap_or_default(),
                    minion.variant.data().name
                ),
            )
        })
        .collect();

    //TODO: better replay format
    assert_yaml_snapshot!(
        format!("random_board_snapshot_{}", seed),
        (minions, recording.events.queue)
    );
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
            let minion = game.instantiate_minion(MinionVariant::random(&mut rng), false);
            game.position_minion(minion, PlayerId::Bottom).unwrap();

            let minion = game.instantiate_minion(MinionVariant::random(&mut rng), false);
            game.position_minion(minion, PlayerId::Top).unwrap();
        }

        for (_mi_id, minion) in game.minion_instances {
            variants_left.remove(&minion.variant);
        }
    }

    assert!(variants_left.len() == 0, "{}/{} not covered.", variants_left.len(), variant_count)
}

#[test]
pub fn random_board_outcomes() {
    let mut outcomes = Vec::new();

    for seed in 0..1000 {
        let mut game = Game::with_seed(seed);

        let mut rng = Xoshiro256PlusPlus::seed_from_u64(!seed);

        for _ in 0..7 {
            let minion = game.instantiate_minion(MinionVariant::random(&mut rng), rng.gen());
            game.position_minion(minion, PlayerId::Bottom).unwrap();

            let minion = game.instantiate_minion(MinionVariant::random(&mut rng), rng.gen());
            game.position_minion(minion, PlayerId::Top).unwrap();
        }

        game.initialize();

        let result = game.run();

        #[derive(Serialize, Deserialize)]
        enum Outcome {
            Draw,
            BottomWin(FinalBoard),
            TopWin(FinalBoard),
        }

        #[derive(Serialize, Deserialize)]
        struct FinalBoard {
            minions: Vec<Minion>,
        }

        #[derive(Serialize, Deserialize)]
        struct Minion {
            name: String,
            attack: i32,
            health: i32,
        }

        fn minions_from_player(game: &Game, player_id: PlayerId) -> Vec<Minion> {
            let minions = game
                .battleground
                .player(player_id)
                .board
                .minions
                .iter()
                .map(|mi_id| {
                    let minion = game.minion_instances.get(*mi_id).unwrap();
                    Minion {
                        name: minion.variant.data().name,
                        attack: minion.attack(),
                        health: minion.health(),
                    }
                })
                .collect();
            minions
        }

        let outcome = match result {
            End::Draw => Outcome::Draw,
            End::BottomWin => {
                let minions = minions_from_player(&game, PlayerId::Bottom);
                Outcome::BottomWin(FinalBoard {
                    minions,
                })
            }
            End::TopWin => {
                let minions = minions_from_player(&game, PlayerId::Top);
                Outcome::TopWin(FinalBoard {
                    minions,
                })
            }
        };
        outcomes.push((seed, outcome));
    }

    let results_map: BTreeMap<_, _> = outcomes.into_iter().collect();
    assert_yaml_snapshot!("random_board_outcomes", (results_map));
}

//TODO: test implemented minions more targeted to reduce the unnecessary hight amount of tests in this state
