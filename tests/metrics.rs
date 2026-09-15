mod common;

use insta::assert_debug_snapshot;
use search_algorithms::minimax::*;
use common::test_file_iterator;
use search_algorithms::connect4::Connect4Basic; 

macro_rules! metrics_tests {
    ($mod_name:ident, $file:expr, [$($algo_name:ident => $algo_func:expr),* $(,)?]) => {
        mod $mod_name {
            #[allow(unused_imports)]
            use super::*;
            
            $(
                #[test]
                #[ignore]
                fn $algo_name() {
                    let mut nodes_visited = 0;
                    let mut n_tests = 0;
                    
                    for (seq, _) in test_file_iterator($file) {
                        let mut board = Connect4Basic::from_sequence(&seq).unwrap();
                        
                        $algo_func(&mut board, &mut |event| match event {
                            MetricEvent::NodeVisited => nodes_visited += 1,
                            MetricEvent::PruningTriggered => (),
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

metrics_tests!(l3_r1, "tests/Test_L3_R1", [
    basic => negamax,
    half_pruned => negamax_half_pruned,
    pruned => negamax_pruned,
]);

metrics_tests!(l2_r1, "tests/Test_L2_R1", [
    // half_pruned => negamax_half_pruned,
    // pruned => negamax_pruned,
]);

metrics_tests!(l2_r2, "tests/Test_L2_R2", [
    // half_pruned => negamax_half_pruned,
    // pruned => negamax_pruned,
]);
