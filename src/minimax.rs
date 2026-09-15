use crate::searchtree::ZeroSumTree;
use std::ops::ControlFlow;

pub fn negamax<T: ZeroSumTree>(st: &mut T) -> (T::Score, Option<T::Move>) {
    let mut best_score = None;
    let mut best_game_move  = None;
    
    let terminal_score = st.evaluate(|game_move, ct| {
        let (score, _) = negamax(ct);
        let score = -score;

        if best_score.is_none_or(|best_score| best_score < score) {
            best_score = Some(score);
            best_game_move = Some(game_move);
        }

        ControlFlow::Continue(())
    });
    
    match terminal_score {
        Some(score) => (score, None),
        None => (best_score.expect("Non terminal state should have at least move"), best_game_move)
    }
}

fn negamax_half_pruned_core<T: ZeroSumTree>(st: &mut T, opp_best_score: Option<T::Score>) -> (T::Score, Option<T::Move>) {

    let mut best_score = None;
    let mut best_game_move  = None;

    let terminal_score = st.evaluate(|game_move, ct| {
        let (opp_score, _) = negamax_half_pruned_core(ct, best_score);

        let score = -opp_score;

        // DON'T TAKE EQUAL SCORES
        if best_score.is_none_or(|best_score| best_score < score) {
            best_score = Some(score);
            best_game_move = Some(game_move);
        }

        if let Some(a) = opp_best_score
            && opp_score <= a // SKIP EVEN IF EQUAL
        {
            best_game_move = None;
            return ControlFlow::Break(());
        }

        ControlFlow::Continue(())
    });

    match terminal_score {
        Some(score) => (score, None),
        None => (best_score.expect("Non terminal state should have at least move"), best_game_move)
    }
}

pub fn negamax_half_pruned<T: ZeroSumTree>(st: &mut T) -> (T::Score, Option<T::Move>) {
    negamax_half_pruned_core(st, None)
}

fn negamax_pruned_core<T: ZeroSumTree>(st: &mut T, opp_best_score: Option<T::Score>, mut prev_best_score: Option<T::Score>) -> (T::Score, Option<T::Move>) {

    let mut best_score = None;
    let mut best_game_move  = None;

    let terminal_score = st.evaluate(|game_move, ct| {
        let (opp_score, _) = negamax_pruned_core(ct, prev_best_score, opp_best_score);

        let score = -opp_score;

        // DON'T TAKE EQUAL SCORES
        if best_score.is_none_or(|best_score| best_score < score) {
            best_score = Some(score);
            best_game_move = Some(game_move);
        }

        // Prune?
        if let Some(a) = opp_best_score
            && opp_score <= a // SKIP EVEN IF EQUAL
        {
            best_game_move = None;
            return ControlFlow::Break(());
        }

        // New best for this line?
        if prev_best_score.is_none_or(|prev_best_score| prev_best_score < score) {
            prev_best_score = Some(score);
        }

        ControlFlow::Continue(())
    });

    match terminal_score {
        Some(score) => (score, None),
        None => (best_score.expect("Non terminal state should have at least move"), best_game_move)
    }
}

pub fn negamax_pruned<T: ZeroSumTree>(st: &mut T) -> (T::Score, Option<T::Move>) {
    negamax_pruned_core(st, None, None)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::searchtree::ZeroSumTree;

    #[derive(Clone, Copy, PartialEq, Debug)]
    enum MockMove {
        Left,
        Right,
    }

    /// A mock game tree to test negamax in isolation.
    /// Tree structure (scores are from the perspective of Player 1 at depth 2):
    ///          0 (P1)
    ///        /       \
    ///      1 (P2)     2 (P2)
    ///     /   \      /   \
    ///   3(10) 4(5) 5(-10) 6(7)
    ///
    /// P2 at node 1 will choose node 4 (score 5) because P2 wants to minimize P1's score.
    /// P2 at node 2 will choose node 5 (score -10) for the same reason.
    /// P1 at node 0 will choose node 1 (which leads to score 5) rather than node 2 (leads to -10).
    struct MockGame {
        node: usize,
        history: Vec<usize>,
    }

    impl ZeroSumTree for MockGame {
        type Move = MockMove;

        fn get_moves(&self) -> Vec<Self::Move> {
            if self.node >= 3 {
                vec![] // Terminal nodes have no moves
            } else {
                vec![MockMove::Left, MockMove::Right]
            }
        }

        fn make_move(&mut self, game_move: &Self::Move) {
            self.history.push(self.node);
            self.node = match game_move {
                MockMove::Left => 2 * self.node + 1,
                MockMove::Right => 2 * self.node + 2,
            };
        }

        fn undo_move(&mut self, _game_move: &Self::Move) {
            self.node = self.history.pop().unwrap();
        }
    }

    impl ZeroSumGame for MockGame {
        type Score = i32;

        fn terminal_score(&self) -> Option<Self::Score> {
            match self.node {
                3 => Some(10),
                4 => Some(5),
                5 => Some(-10),
                6 => Some(7),
                _ => None, // Non-terminal nodes return None
            }
        }
    }

    #[test]
    fn test_negamax_chooses_correct_path() {
        let mut game = MockGame { node: 0, history: Vec::new() };
        let (score, best_move) = negamax(&mut game);
        
        // P1 should choose Left (leading to node 1 -> node 4 -> score 5)
        assert_eq!(score, 5);
        assert_eq!(best_move, Some(MockMove::Left));
    }

    #[test]
    fn test_negamax_half_pruned_chooses_correct_path() {
        let mut game = MockGame { node: 0, history: Vec::new() };
        let (score, best_move) = negamax_half_pruned(&mut game);
        
        // P1 should choose Left (leading to node 1 -> node 4 -> score 5)
        assert_eq!(score, 5);
        assert_eq!(best_move, Some(MockMove::Left));
    }

    #[test]
    fn test_negamax_pruned_chooses_correct_path() {
        let mut game = MockGame { node: 0, history: Vec::new() };
        let (score, best_move) = negamax_pruned(&mut game);
        
        // P1 should choose Left (leading to node 1 -> node 4 -> score 5)
        assert_eq!(score, 5);
        assert_eq!(best_move, Some(MockMove::Left));
    }
}

