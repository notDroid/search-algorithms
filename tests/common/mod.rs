#![allow(dead_code)]
use std::fs;
use search_algorithms::connect4::Connect4;
use search_algorithms::searchtree::{ReversibleGame, ZeroSumGame};

// METRICS WRAPPER
pub struct InstrumentedGame<G> {
    pub inner: G,
    pub moves_played: usize,
}

impl<G> InstrumentedGame<G> {
    pub fn new(inner: G) -> Self {
        Self { inner, moves_played: 0 }
    }
}

impl<G: ReversibleGame> ReversibleGame for InstrumentedGame<G> {
    type Move = G::Move;

    fn get_moves(&self) -> Vec<Self::Move> {
        self.inner.get_moves()
    }

    fn make_move(&mut self, game_move: &Self::Move) {
        self.moves_played += 1;
        self.inner.make_move(game_move);
    }

    fn undo_move(&mut self, game_move: &Self::Move) {
        self.inner.undo_move(game_move);
    }
}

impl<G: ZeroSumGame> ZeroSumGame for InstrumentedGame<G> {
    type Score = G::Score;

    fn terminal_score(&self) -> Option<Self::Score> {
        self.inner.terminal_score()
    }
}

// FILE PARSING HELPER
fn test_file_iterator(file_path: &str) -> Vec<(String, i32)> {
    let contents = fs::read_to_string(file_path).expect("Failed to read file");

    contents
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();

            (parts[0].to_string(), parts[1].parse().unwrap())
        }).collect()
}

// CORRECTNESS HELPER
pub fn run_test_file<F>(file_path: &str, mut solver: F)
where
    F: FnMut(&mut Connect4) -> (i32, Option<<Connect4 as ReversibleGame>::Move>),
{    
    for (line_num, (seq, expected_score)) in test_file_iterator(file_path).into_iter().enumerate() {

        let mut board = Connect4::from_sequence(&seq).expect("valid sequence");
        let (score, _) = solver(&mut board);

        assert_eq!(
            score, expected_score, 
            "Failed on line {}: sequence {}", line_num + 1, seq
        );
    }
}

// METRICS HELPER
pub fn run_metrics_file<F>(file_path: &str, mut solver: F) -> f64
where
    F: FnMut(&mut InstrumentedGame<Connect4>) -> (i32, Option<<Connect4 as ReversibleGame>::Move>),
{
    
    let mut total_moves = 0;
    let mut count = 0;

    for (seq, _) in test_file_iterator(file_path).into_iter() {
        let board = Connect4::from_sequence(&seq).expect("valid sequence");
        let mut instrumented = InstrumentedGame::new(board);
        
        solver(&mut instrumented);
        
        total_moves += instrumented.moves_played;
        count += 1;
    }

    if count == 0 { 0.0 } else { total_moves as f64 / count as f64 }
}
