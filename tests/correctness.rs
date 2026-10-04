mod common;

use common::test_file_iterator;
use search_algorithms::connect4::{Connect4BasicTree, Connect4BitBoardTreeDefaultOrder, Connect4BitBoardTreeCenterOrder};
use search_algorithms::minimax::*;

macro_rules! correctness_tests {
    ($mod_name:ident, $file:expr, $board:ty, [$($algo_name:ident => $algo_func:expr),* $(,)?]) => {
        mod $mod_name {
            #[allow(unused_imports)]
            use super::*;

            $(
                #[test]
                #[ignore]
                fn $algo_name() {
                    for (line_num, (seq, expected)) in test_file_iterator($file).into_iter().enumerate() {
                        let mut board = <$board>::from_sequence(&seq).unwrap();
                        dbg!(line_num);
                        let (score, _) = $algo_func(&mut board, &mut |_| {});

                        assert_eq!(score, expected, "Failed on line {}: sequence {}", line_num + 1, seq);
                    }
                }
            )*
        }
    };
}

#[test]
fn connect4_simple_basic() {
    let seq = "2252576253462244111563365343671351441";
    let expected_score: i32 = -1;

    let mut board = Connect4BasicTree::from_sequence(seq).expect("valid sequence");
    let (score, _best_move) = negamax(&mut board, &mut |_| {});

    assert_eq!(score, expected_score);
}

#[test]
fn connect4_simple_bitboard() {
    let seq = "2252576253462244111563365343671351441";
    let expected_score: i32 = -1;

    let mut board = Connect4BitBoardTreeDefaultOrder::from_sequence(seq).expect("valid sequence");
    let (score, _best_move) = negamax(&mut board, &mut |_| {});

    assert_eq!(score, expected_score);
}

#[test]
fn connect4_simple_bitboard_center() {
    let seq = "2252576253462244111563365343671351441";
    let expected_score: i32 = -1;

    let mut board = Connect4BitBoardTreeCenterOrder::from_sequence(seq).expect("valid sequence");
    let (score, _best_move) = negamax(&mut board, &mut |_| {});

    assert_eq!(score, expected_score);
}

correctness_tests!(l3_r1_basic, "tests/Test_L3_R1", Connect4BasicTree, [
    basic => negamax,
    half_pruned => negamax_half_pruned,
    pruned => negamax_pruned,
]);

correctness_tests!(l3_r1_bitboard, "tests/Test_L3_R1", Connect4BitBoardTreeDefaultOrder, [
    basic => negamax,
    half_pruned => negamax_half_pruned,
    pruned => negamax_pruned,
    alpha_beta => negamax_alpha_beta,
    pruned_trans_hashmap => negamax_pruned_trans_lower0,
    alpha_beta_trans => negamax_alpha_beta_trans_lower0,
    // pruned_trans_kv => negamax_pruned_trans_lower1,
    pruned_trans_kv_fh => negamax_pruned_trans_lower2,
]);

correctness_tests!(l3_r1_ordered, "tests/Test_L3_R1", Connect4BitBoardTreeCenterOrder, [
    alpha_beta => negamax_alpha_beta,
    pruned_trans_kv_fh_83 => negamax_pruned_trans_lower2_83,
]);

correctness_tests!(l2_r1_bitboard, "tests/Test_L2_R1", Connect4BitBoardTreeDefaultOrder, [
    // half_pruned => negamax_half_pruned,
    // pruned => negamax_pruned,
    pruned_trans => negamax_pruned_trans_lower0,
    pruned_trans_kv_fh => negamax_pruned_trans_lower2,
]);

correctness_tests!(l2_r2_bitboard, "tests/Test_L2_R2", Connect4BitBoardTreeDefaultOrder, [
    // half_pruned => negamax_half_pruned,
    // pruned => negamax_pruned,
    pruned_trans => negamax_pruned_trans_lower0,
    pruned_trans_kv_fh => negamax_pruned_trans_lower2_83,
]);
