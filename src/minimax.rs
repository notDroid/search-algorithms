use crate::game;

pub fn negamax<T: game::ZeroSumGame>(game: &mut T) -> (T::Score, Option<T::Move>) {
    if let Some(score) = game.terminal_score() {
        return (score, None);
    }

    let mut best_score = None;
    let mut best_game_move  = None;

    for game_move in game.get_moves() {
        game.make_move(&game_move);
        let (score, _) = negamax(game);
        let score = -score;
        game.undo_move(&game_move);

        if best_score.is_none_or(|best_score| best_score < score) {
            best_score = Some(score);
            best_game_move = Some(game_move);
        }
    }

    (best_score.expect("Terminal should have at least move"), best_game_move)
}
