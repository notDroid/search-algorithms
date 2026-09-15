#![allow(dead_code)]
use std::fs;
use search_algorithms::connect4::Connect4Basic;
use search_algorithms::minimax::MetricEvent;
use search_algorithms::searchtree::SearchTree;

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
    F: FnMut(&mut Connect4Basic, &mut dyn FnMut(MetricEvent)) -> (i32, Option<<Connect4Basic as SearchTree>::Move>),
{
    
    let mut nodes_visited = 0;
    let mut n_tests = 0;

    for (seq, _) in test_file_iterator(file_path).into_iter() {
        let mut board = Connect4Basic::from_sequence(&seq).expect("valid sequence");
        let mut metrics = |event: MetricEvent| {
            match event {
                MetricEvent::NodeVisited => nodes_visited += 1,
                MetricEvent::PruningTriggered => todo!(),
            }
        };
        
        solver(&mut board, &mut metrics);
        n_tests += 1;
    }

    nodes_visited as f64 / n_tests as f64
}