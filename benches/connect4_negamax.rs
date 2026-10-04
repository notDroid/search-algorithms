use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use search_algorithms::connect4::{Connect4Basic, Connect4BitBoard};
use search_algorithms::minimax::{
    negamax, negamax_alpha_beta_trans_lower0, negamax_half_pruned, negamax_pruned,
    negamax_pruned_trans_lower0, negamax_pruned_trans_lower1,
};
use std::fs;
use std::hint::black_box;

macro_rules! bench_algorithms {
    ($c:expr, $group_name:expr, $file:expr, $board:ty, [$($algo_name:ident => $algo_func:expr),* $(,)?]) => {
        {
            let mut group = $c.benchmark_group($group_name);
            group.sample_size(10);

            $(
                group.bench_function(stringify!($algo_name), |b| {
                    let contents = fs::read_to_string($file).expect("Failed to read file");
                    let games: Vec<$board> = contents
                        .lines()
                        .take(10)
                        .map(|line| line.split_whitespace().next().expect("Expected sequence"))
                        .map(|seq| <$board>::from_sequence(seq).expect("Expected to be able to parse test sequence into game object"))
                        .collect();

                    b.iter_batched(
                        || games.clone(),
                        |mut cloned_games| {
                            for g in cloned_games.iter_mut() {
                                $algo_func(black_box(g), black_box(&mut |_| {}));
                            }
                        },
                        BatchSize::SmallInput,
                    )
                });
            )*

            group.finish();
        }
    };
}

pub fn criterion_benchmark(c: &mut Criterion) {
    bench_algorithms!(c, "connect4_L3_R1_basic", "tests/Test_L3_R1", Connect4Basic, [
        negamax => negamax,
        negamax_half_pruned => negamax_half_pruned,
        negamax_pruned => negamax_pruned,
    ]);

    bench_algorithms!(c, "connect4_L3_R1_bitboard", "tests/Test_L3_R1", Connect4BitBoard, [
        negamax => negamax,
        negamax_half_pruned => negamax_half_pruned,
        negamax_pruned => negamax_pruned,
        negamax_pruned_trans => negamax_pruned_trans_lower0,
    ]);

    bench_algorithms!(c, "connect4_L2_R1_bitboard", "tests/Test_L2_R1", Connect4BitBoard, [
        // negamax => negamax,
        // negamax_half_pruned => negamax_half_pruned,
        negamax_pruned => negamax_pruned,
        negamax_pruned_trans => negamax_pruned_trans_lower0,
        alpha_beta_trans => negamax_alpha_beta_trans_lower0,
        negamax_pruned_trans_kv => negamax_pruned_trans_lower1,
    ]);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
