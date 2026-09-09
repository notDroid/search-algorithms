pub trait ReversibleGame {
    type Move;

    fn get_moves(&self) -> Vec<Self::Move>;
    fn make_move(&mut self, game_move: &Self::Move);
    fn undo_move(&mut self, game_move: &Self::Move);
}

pub trait ZeroSumGame: ReversibleGame {
    type Score: Ord + Copy + std::ops::Neg<Output = Self::Score>;

    fn terminal_score(&self) -> Option<Self::Score>;
}