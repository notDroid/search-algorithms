use std::fs;
use search_algorithms::connect4::Connect4;
use search_algorithms::minimax::*;
use search_algorithms::game::ReversibleGame;

fn run_test_file<F>(file_path: &str, mut solver: F)
where
    F: FnMut(&mut Connect4) -> (i32, Option<<Connect4 as ReversibleGame>::Move>),
{
    let contents = fs::read_to_string(file_path).expect("Failed to read file");
    
    for (line_num, line) in contents.lines().enumerate() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 2 { continue; }
        
        let seq = parts[0];
        let expected_score: i32 = parts[1].parse().unwrap();

        let mut board = Connect4::from_sequence(seq).expect("valid sequence");
        let (score, _) = solver(&mut board);

        assert_eq!(
            score, expected_score, 
            "Failed on line {}: sequence {}", line_num + 1, seq
        );
    }
}

#[test]
fn connect4_simple() {
    let seq = "2252576253462244111563365343671351441";
    let expected_score: i32 = -1;

    let mut board = Connect4::from_sequence(seq).expect("valid sequence");
    let (score, _best_move) = negamax(&mut board);

    assert_eq!(score, expected_score);
}

#[test]
#[ignore]
fn test_negamax_l3_r1() {
    run_test_file("tests/Test_L3_R1", |board| negamax(board));
}

#[test]
#[ignore]
fn test_negamax_pruned_l3_r1() {
    run_test_file("tests/Test_L3_R1", |board| negamax_pruned(board));
}