use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use std::hint::black_box;
use std::fs;
use search_algorithms::connect4::Connect4;
use search_algorithms::minimax::{negamax, negamax_pruned, negamax_half_pruned};

pub fn criterion_benchmark(c: &mut Criterion) {
    let contents = fs::read_to_string("tests/Test_L3_R1").expect("Failed to read file");

    let games: Vec<Connect4> = contents
        .lines()
        .map(|line| line.split_whitespace().next().expect("Expected sequence"))
        .map(|seq| Connect4::from_sequence(seq).expect("Expected to be able to parse test sequence into game object"))
        .collect();

    let mut group = c.benchmark_group("connect4_benchmarks");
    group.sample_size(10); 
    
    group.bench_function("negamax_L3_R1", |b| {
        b.iter_batched(
            || games.clone(),
            |mut cloned_games| {
                for g in cloned_games.iter_mut() {
                    negamax(black_box(g));
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("negamax_half_pruned_L3_R1", |b| {
        b.iter_batched(
            || games.clone(),
            |mut cloned_games| {
                for g in cloned_games.iter_mut() {
                    negamax_half_pruned(black_box(g));
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("negamax_pruned_L3_R1", |b| {
        b.iter_batched(
            || games.clone(),
            |mut cloned_games| {
                for g in cloned_games.iter_mut() {
                    negamax_pruned(black_box(g));
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
