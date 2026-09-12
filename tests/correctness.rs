mod common;

use search_algorithms::connect4::Connect4;
use search_algorithms::minimax::*;
use common::run_test_file;

#[test]
fn connect4_simple() {
    let seq = "2252576253462244111563365343671351441";
    let expected_score: i32 = -1;

    let mut board = Connect4::from_sequence(seq).expect("valid sequence");
    let (score, _best_move) = negamax(&mut board);

    assert_eq!(score, expected_score);
}

#[test]
#[ignore]
fn test_negamax_l3_r1() {
    run_test_file("tests/Test_L3_R1", |board| negamax(board));
}

#[test]
#[ignore]
fn test_negamax_half_pruned_l3_r1() {
    run_test_file("tests/Test_L3_R1", |board| negamax_half_pruned(board));
}

#[test]
#[ignore]
fn test_negamax_pruned_l3_r1() {
    run_test_file("tests/Test_L3_R1", |board| negamax_pruned(board));
}
