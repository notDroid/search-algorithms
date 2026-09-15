#![allow(dead_code)]
use std::fs;
use search_algorithms::connect4::Connect4Basic;
use search_algorithms::searchtree::SearchTree;

// METRICS WRAPPER
pub struct MoveCountedTree<G> {
    pub inner: G,
    pub moves_played: usize,
}

impl<G> MoveCountedTree<G> {
    pub fn new(inner: G) -> Self {
        Self { inner, moves_played: 0 }
    }
}

impl<G: SearchTree> SearchTree for MoveCountedTree<G> {
    type Score = G::Score;
    type Move = G::Move;
    
    fn evaluate<F>(&mut self, on_ongoing: F) -> Option<Self::Score>
    where
        F: FnMut(Self::Move, &mut Self) -> std::ops::ControlFlow<()> {
        
        self.inner.evaluate(|x, y| {
            let mut counter = MoveCountedTree::new(y);
            let control = on_ongoing(x, &mut counter);
            self.moves_played += 1 + counter.moves_played;
            control
        })
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
    F: FnMut(&mut Connect4Basic) -> (i32, Option<<Connect4Basic as SearchTree>::Move>),
{    
    for (line_num, (seq, expected_score)) in test_file_iterator(file_path).into_iter().enumerate() {

        let mut board = Connect4Basic::from_sequence(&seq).expect("valid sequence");
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
    F: FnMut(&mut InstrumentedGame<Connect4Basic>) -> (i32, Option<<Connect4Basic as SearchTree>::Move>),
{
    
    let mut total_moves = 0;
    let mut count = 0;

    for (seq, _) in test_file_iterator(file_path).into_iter() {
        let board = Connect4Basic::from_sequence(&seq).expect("valid sequence");
        let mut instrumented = InstrumentedGame::new(board);
        
        solver(&mut instrumented);
        
        total_moves += instrumented.moves_played;
        count += 1;
    }

    if count == 0 { 0.0 } else { total_moves as f64 / count as f64 }
}
