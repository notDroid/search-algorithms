use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use std::hint::black_box;
use std::fs;
use search_algorithms::connect4::Connect4;
use search_algorithms::minimax::negamax;

pub fn criterion_benchmark(c: &mut Criterion) {
    let contents = fs::read_to_string("tests/Test_L3_R1").expect("Failed to read file");

    let games: Vec<Connect4> = contents
        .lines()
        .map(|line| line.split_whitespace().next().expect("Expected sequence"))
        .map(|seq| Connect4::from_sequence(seq).expect("Expected to be able to parse test sequence into game object"))
        .collect();

    let mut group = c.benchmark_group("connect4_negamax");
    // Reduce sample size since running 1000 tests takes a while
    group.sample_size(10); 
    
    group.bench_function("Test_L3_R1_Suite", |b| {
        b.iter_batched(
            || games.clone(), // Setup: Clone the games before each iteration (not timed)
            |mut cloned_games| {
                // This is the actual code being timed
                for g in cloned_games.iter_mut() {
                    negamax(black_box(g));
                }
            },
            BatchSize::SmallInput,
        )
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
