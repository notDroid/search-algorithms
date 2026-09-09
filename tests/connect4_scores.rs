use std::fs;
// use search_algorithms::connect4::Connect4;
use search_algorithms::minimax::negamax;

#[test]
fn connect4_l3_r1_first_case() {
    let content = fs::read_to_string("tests/Test_L3_R1").unwrap();
    let first_line = content.lines().next().expect("file should not be empty");
    let mut parts = first_line.split(' ');
    let seq = parts.next().expect("seq missing");
    let expected_score: i32 = parts.next().expect("score missing").parse().unwrap();

    // let mut board = Connect4::from_sequence(seq).expect("valid sequence");
    // let (score, _best_move) = negamax(&mut board);

    // assert_eq!(score, expected_score);
}