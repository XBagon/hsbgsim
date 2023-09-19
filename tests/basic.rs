extern crate hsbgsim;

use std::{env, str::FromStr};

use hsbgsim::*;

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
pub fn full_maexxna() {
    let mut game = Game::default();

    for _ in 0..7 {
        let maexxna = game.instantiate_minion(MinionVariant::Maexxna);
        game.position_minion(maexxna, PlayerId::Bottom).unwrap();
    }

    for _ in 0..7 {
        let maexxna = game.instantiate_minion(MinionVariant::Maexxna);
        game.position_minion(maexxna, PlayerId::Top).unwrap();
    }

    game.initialize();

    assert_eq!(game.run(), End::Draw);
}

#[test]
pub fn full_maexxna_vs_amalgam() {
    let mut game = Game::default();

    for _ in 0..7 {
        let maexxna = game.instantiate_minion(MinionVariant::Maexxna);
        game.position_minion(maexxna, PlayerId::Bottom).unwrap();
    }

    for _ in 0..7 {
        let amalgam = game.instantiate_minion(MinionVariant::Amalgam);
        game.position_minion(amalgam, PlayerId::Top).unwrap();
    }

    game.initialize();

    assert_eq!(game.run(), End::Draw);
}

#[test]
pub fn top_bottom_fair() {
    let mut top_wins = 0;
    let mut bottom_wins = 0;

    for _ in 0..10000 {
        let mut game = Game::default();

        let mut rng = rand::thread_rng();

        for _ in 0..7 {
            let minion = game.instantiate_minion(MinionVariant::random(&mut rng));
            game.position_minion(minion, PlayerId::Bottom).unwrap();

            let minion = game.instantiate_minion(MinionVariant::random(&mut rng));
            game.position_minion(minion, PlayerId::Top).unwrap();
        }

        game.initialize();

        match game.run() {
            End::Draw => {}
            End::BottomWin => bottom_wins += 1,
            End::TopWin => top_wins += 1,
        }
    }

    let ratio = top_wins as f32 / bottom_wins as f32;
    let deviation = (ratio - 1.0).abs();
    assert!(deviation <= 0.1, "deviation was {}", deviation);
}
