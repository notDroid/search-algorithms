use std::ops::{ControlFlow, Neg};
use std::hash::Hash;

pub trait SearchTree {
    type Move;
    type Score: Ord;

    /// Evaluates the current state.
    /// - If the game is over, it returns `Some(score)`.
    /// - If the game is ongoing, it feeds all child states into the provided closure,
    ///   and then returns `None`.
    fn evaluate<F>(&mut self, on_ongoing: F) -> Option<Self::Score>
    where
        F: FnMut(Self::Move, &mut Self) -> ControlFlow<()>;
}

pub trait ZeroSumTree: SearchTree<Score = Self::ZScore> {
    type ZScore: Copy + Ord + Neg<Output = Self::ZScore>;
}

pub trait StateKey: SearchTree {
    type Key: Hash + Eq + Copy;

    fn key(&self) -> Self::Key;
}
