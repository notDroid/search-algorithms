use std::fs;
use search_algorithms::connect4::*;
use search_algorithms::minimax::negamax;

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
fn test_entire_l3_r1() {
    let contents = fs::read_to_string("tests/Test_L3_R1").expect("Failed to read file");
    
    for (line_num, line) in contents.lines().enumerate() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 2 { continue; }
        
        let seq = parts[0];
        let expected_score: i32 = parts[1].parse().unwrap();

        let mut board = Connect4::from_sequence(seq).expect("valid sequence");
        let (score, _) = negamax(&mut board);

        assert_eq!(
            score, expected_score, 
            "Failed on line {}: sequence {}", line_num + 1, seq
        );
    }
}