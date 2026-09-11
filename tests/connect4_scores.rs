use search_algorithms::connect4::Connect4;
use search_algorithms::minimax::negamax;

#[test]
fn connect4_l3_r1_first_case() {
    let seq = "2252576253462244111563365343671351441";
    let expected_score: i32 = -1;

    let mut board = Connect4::from_sequence(seq).expect("valid sequence");
    dbg!(&board);
    let (score, _best_move) = negamax(&mut board);

    assert_eq!(score, expected_score);
}
