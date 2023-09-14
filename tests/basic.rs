extern crate hsbgsim;

use hsbgsim::*;
use tinyvec::array_vec;

#[test]
pub fn empty() {
    let mut game = Game::default();
    game.initialize();
    game.run_and_print();
}

#[test]
pub fn single_annoy_o_trons() {
    let mut game = Game::default();

    let annoy_o_tron = game.instantiate_minion(MinionVariant::AnnoyOTron);
    game.position_minion(annoy_o_tron, PlayerId::Bottom).unwrap();

    let annoy_o_tron = game.instantiate_minion(MinionVariant::AnnoyOTron);
    game.position_minion(annoy_o_tron, PlayerId::Top).unwrap();

    game.initialize();

    game.run_and_print();
}

#[test]
pub fn full_annoy_o_trons() {
    let mut game = Game::default();

    for _ in 0..7 {
        let annoy_o_tron = game.instantiate_minion(MinionVariant::AnnoyOTron);
        game.position_minion(annoy_o_tron, PlayerId::Bottom).unwrap();

        let annoy_o_tron = game.instantiate_minion(MinionVariant::AnnoyOTron);
        game.position_minion(annoy_o_tron, PlayerId::Top).unwrap();
    }

    game.initialize();

    game.run_and_print();
}

#[test]
pub fn two_vs_five_annoy_o_trons() {
    let mut game = Game::default();

    for _ in 0..2 {
        let annoy_o_tron = game.instantiate_minion(MinionVariant::AnnoyOTron);
        game.position_minion(annoy_o_tron, PlayerId::Bottom).unwrap();
    }

    for _ in 0..5 {
        let annoy_o_tron = game.instantiate_minion(MinionVariant::AnnoyOTron);
        game.position_minion(annoy_o_tron, PlayerId::Top).unwrap();
    }

    game.initialize();

    game.run_and_print();
}

#[test]
pub fn full_random() {
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
