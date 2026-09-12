mod common;

use insta::assert_debug_snapshot;
use search_algorithms::minimax::*;
use common::run_metrics_file;

#[test]
#[ignore]
fn metrics_negamax_l3_r1() {
    let avg = run_metrics_file("tests/Test_L3_R1", |board| negamax(board));
    assert_debug_snapshot!("negamax_L3_R1_avg_moves", avg);
}

#[test]
#[ignore]
fn metrics_negamax_half_pruned_l3_r1() {
    let avg = run_metrics_file("tests/Test_L3_R1", |board| negamax_half_pruned(board));
    assert_debug_snapshot!("negamax_half_pruned_L3_R1_avg_moves", avg);
}

#[test]
#[ignore]
fn metrics_negamax_pruned_l3_r1() {
    let avg = run_metrics_file("tests/Test_L3_R1", |board| negamax_pruned(board));
    assert_debug_snapshot!("negamax_pruned_L3_R1_avg_moves", avg);
}
