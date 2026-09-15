mod common;

use insta::assert_debug_snapshot;
use search_algorithms::minimax::*;
use common::run_metrics_file;

macro_rules! metrics_tests {
    ($mod_name:ident, $file:expr, [$($algo_name:ident => $algo_func:expr),* $(,)?]) => {
        mod $mod_name {
            use super::*;
            $(
                #[test]
                #[ignore]
                fn $algo_name() {
                    let avg = run_metrics_file($file, |board, m| $algo_func(board, m));
                    assert_debug_snapshot!(stringify!($algo_name), avg);
                }
            )*
        }
    };
}

metrics_tests!(l3_r1, "tests/Test_L3_R1", [
    basic => negamax,
    // half_pruned => negamax_half_pruned,
    // pruned => negamax_pruned,
]);

metrics_tests!(l2_r1, "tests/Test_L2_R1", [
    // half_pruned => negamax_half_pruned,
    // pruned => negamax_pruned,
]);

metrics_tests!(l2_r2, "tests/Test_L2_R2", [
    // half_pruned => negamax_half_pruned,
    // pruned => negamax_pruned,
]);
