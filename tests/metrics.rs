mod common;

use common::test_file_iterator;
use insta::assert_debug_snapshot;
use search_algorithms::connect4::Connect4BitBoard;
use search_algorithms::minimax::*;

macro_rules! metrics_tests {
    ($mod_name:ident, $file:expr, $board:ty, [$($algo_name:ident => $algo_func:expr),* $(,)?]) => {
        mod $mod_name {
            #[allow(unused_imports)]
            use super::*;

            $(
                #[test]
                #[ignore]
                fn $algo_name() {
                    let mut nodes_visited: u64 = 0;
                    let mut n_tests = 0;

                    for (seq, _) in test_file_iterator($file) {
                        let mut board = <$board>::from_sequence(&seq).unwrap();

                        $algo_func(&mut board, &mut |event| match event {
                            MetricEvent::NodeVisited => nodes_visited += 1,
                            _ => (),
                        });

                        n_tests += 1;
                    }

                    let avg = if n_tests == 0 { 0.0 } else { nodes_visited as f64 / n_tests as f64 };
                    assert_debug_snapshot!(stringify!($algo_name), avg);
                }
            )*
        }
    };
}

metrics_tests!(l3_r1, "tests/Test_L3_R1", Connect4BitBoard, [
    basic => negamax,
    half_pruned => negamax_half_pruned,
    pruned => negamax_pruned,
    alpha_beta => negamax_alpha_beta,
    pruned_trans => negamax_pruned_trans_lower0,
    alpha_beta_trans => negamax_alpha_beta_trans_lower0,
    // pruned_trans_kv => negamax_pruned_trans_lower1,
    pruned_trans_kv_fh => negamax_pruned_trans_lower2,
    pruned_trans_kv_fh_83 => negamax_pruned_trans_lower2_83
]);

metrics_tests!(
    l2_r1,
    "tests/Test_L2_R1",
    Connect4BitBoard,
    [
    // half_pruned => negamax_half_pruned,
    // pruned => negamax_pruned,
    // alpha_beta_trans => negamax_alpha_beta_trans_lower0,
    alpha_beta_trans => negamax_alpha_beta_trans_lower0,
    pruned_trans_kv_fh_83 => negamax_pruned_trans_lower2_83
]
);

metrics_tests!(
    l2_r2,
    "tests/Test_L2_R2",
    Connect4BitBoard,
    [
    // half_pruned => negamax_half_pruned,
    // pruned => negamax_pruned,
    // alpha_beta_trans => negamax_alpha_beta_trans_lower0,
    alpha_beta_trans => negamax_alpha_beta_trans_lower0,
    pruned_trans_kv_fh_83 => negamax_pruned_trans_lower2_83
]
);
