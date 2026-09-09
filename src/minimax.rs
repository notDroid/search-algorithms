use crate::game;

pub fn negamax<T: game::ZeroSumGame>(game: &mut T) -> (T::Score, Option<T::Move>) {
    if let Some(score) = game.terminal_score() {
        return (score, None);
    }

    let (score, game_move) = game.get_moves()
        .into_iter()
        .map(|game_move| {
            game.make_move(&game_move);
            let (score, _) = negamax(game);
            game.undo_move(&game_move);

            (-score, game_move)
        })
        .max_by_key(|(score, _)| *score)
        .expect("Game is not terminal, but no legal moves were found!");

    (score, Some(game_move))
}