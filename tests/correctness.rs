mod common;

use search_algorithms::minimax::*;
use common::run_test_file;

macro_rules! correctness_tests {
    ($mod_name:ident, $file:expr, [$($algo_name:ident => $algo_func:expr),* $(,)?]) => {
        mod $mod_name {
            use super::*;
            $(
                #[test]
                #[ignore]
                fn $algo_name() {
                    run_test_file($file, |board| $algo_func(board, &mut |_| {}));
                }
            )*
        }
    };
}

#[test]
fn connect4_simple() {
    let seq = "2252576253462244111563365343671351441";
    let expected_score: i32 = -1;

    let mut board = search_algorithms::connect4::Connect4Basic::from_sequence(seq).expect("valid sequence");
    let (score, _best_move) = negamax(&mut board, &mut |_| {});

    assert_eq!(score, expected_score);
}

correctness_tests!(l3_r1, "tests/Test_L3_R1", [
    basic => negamax,
    // half_pruned => negamax_half_pruned,
    // pruned => negamax_pruned,
]);

correctness_tests!(l2_r1, "tests/Test_L2_R1", [
    // half_pruned => negamax_half_pruned,
    // pruned => negamax_pruned,
]);

correctness_tests!(l2_r2, "tests/Test_L2_R2", [
    // half_pruned => negamax_half_pruned,
    // pruned => negamax_pruned,
]);
