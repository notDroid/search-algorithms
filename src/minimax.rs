use crate::searchtree::{ZeroSumTree, StateKey};
use std::ops::ControlFlow;
use std::collections::HashMap;

pub enum MetricEvent {
    NodeVisited,
    PruningTriggered,
}

pub fn negamax<T: ZeroSumTree, M: FnMut(MetricEvent)>(st: &mut T, m: &mut M) -> (T::Score, Option<T::Move>) {
    m(MetricEvent::NodeVisited);
    let mut best_score = None;
    let mut best_game_move  = None;
    
    let terminal_score = st.evaluate(|game_move, ct| {
        let (score, _) = negamax(ct, m);
        let score = -score;

        if best_score.is_none_or(|best_score| best_score < score) {
            best_score = Some(score);
            best_game_move = Some(game_move);
        }

        ControlFlow::Continue(())
    });
    
    match terminal_score {
        Some(score) => (score, None),
        None => (best_score.expect("Non terminal state should have at least one move"), best_game_move)
    }
}

fn negamax_half_pruned_core<T: ZeroSumTree, M: FnMut(MetricEvent)>(st: &mut T, opp_best_score: Option<T::Score>, m: &mut M) -> (T::Score, Option<T::Move>) {
    m(MetricEvent::NodeVisited);

    let mut best_score = None;
    let mut best_game_move  = None;

    let terminal_score = st.evaluate(|game_move, ct| {
        let (opp_score, _) = negamax_half_pruned_core(ct, best_score, m);

        let score = -opp_score;

        // DON'T TAKE EQUAL SCORES
        if best_score.is_none_or(|best_score| best_score < score) {
            best_score = Some(score);
            best_game_move = Some(game_move);
        }

        if let Some(a) = opp_best_score
            && opp_score <= a // SKIP EVEN IF EQUAL
        {
            m(MetricEvent::PruningTriggered);
            best_game_move = None;
            return ControlFlow::Break(());
        }

        ControlFlow::Continue(())
    });

    match terminal_score {
        Some(score) => (score, None),
        None => (best_score.expect("Non terminal state should have at least one move"), best_game_move)
    }
}

pub fn negamax_half_pruned<T: ZeroSumTree, M: FnMut(MetricEvent)>(st: &mut T, m: &mut M) -> (T::Score, Option<T::Move>) {
    negamax_half_pruned_core(st, None, m)
}

fn negamax_pruned_core<T: ZeroSumTree, M: FnMut(MetricEvent)>(st: &mut T, opp_best_score: Option<T::Score>, mut prev_best_score: Option<T::Score>, m: &mut M) -> (T::Score, Option<T::Move>) {
    m(MetricEvent::NodeVisited);

    let mut best_score = None;
    let mut best_game_move  = None;

    let terminal_score = st.evaluate(|game_move, ct| {
        let (opp_score, _) = negamax_pruned_core(ct, prev_best_score, opp_best_score, m);

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
            m(MetricEvent::PruningTriggered);
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
        None => (best_score.expect("Non terminal state should have at least one move"), best_game_move)
    }
}

pub fn negamax_pruned<T: ZeroSumTree, M: FnMut(MetricEvent)>(st: &mut T, m: &mut M) -> (T::Score, Option<T::Move>) {
    negamax_pruned_core(st, None, None, m)
}

fn negamax_pruned_trans_core<T: ZeroSumTree + StateKey, M: FnMut(MetricEvent)>(transposition_table: &mut HashMap<T::Key, T::Score>, st: &mut T, opp_best_score: Option<T::Score>, mut prev_best_score: Option<T::Score>, m: &mut M) -> (T::Score, Option<T::Move>) {
    m(MetricEvent::NodeVisited);

    let mut best_score = None;
    let mut best_game_move  = None;

    let terminal_score = st.evaluate(|game_move, ct| {
        let key = ct.key();
        let opp_score = match transposition_table.get(&key) {
            Some(opp_score) => *opp_score,
            None => {
                let (opp_score, _) = negamax_pruned_core(ct, prev_best_score, opp_best_score, m);
                transposition_table.insert(key, opp_score);
                opp_score
            }
        };

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
            m(MetricEvent::PruningTriggered);
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
        None => (best_score.expect("Non terminal state should have at least one move"), best_game_move)
    }
}

pub fn negamax_pruned_trans0<T: ZeroSumTree + StateKey, M: FnMut(MetricEvent)>(st: &mut T, m: &mut M) -> (T::Score, Option<T::Move>) {
    let mut transposition_table = HashMap::new();
    negamax_pruned_trans_core(&mut transposition_table, st, None, None, m)
}

pub fn negamax_pruned_trans<T: ZeroSumTree + StateKey, M: FnMut(MetricEvent)>(transposition_table: &mut HashMap<T::Key, T::Score>, st: &mut T, m: &mut M) -> (T::Score, Option<T::Move>) {
    negamax_pruned_trans_core(transposition_table, st, None, None, m)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::searchtree::{SearchTree, ZeroSumTree};

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
    struct MockSearchTree {
        node: usize,
        history: Vec<usize>,
    }

    impl MockSearchTree {
        fn terminal_score(&self) -> Option<i32> {
            match self.node {
                3 => Some(10),
                4 => Some(5),
                5 => Some(-10),
                6 => Some(7),
                _ => None, // Non-terminal nodes return None
            }
        }

        fn get_moves(&self) -> Vec<MockMove> {
            if self.node >= 3 {
                vec![]
            } else {
                vec![MockMove::Left, MockMove::Right]
            }
        }

        fn make_move(&mut self, game_move: MockMove) {
            self.history.push(self.node);
            self.node = match game_move {
                MockMove::Left => 2 * self.node + 1,
                MockMove::Right => 2 * self.node + 2,
            };
        }

        fn undo_move(&mut self, _game_move: MockMove) {
            self.node = self.history.pop().unwrap();
        }

        fn optimal_score(node: usize) -> i32 {
            match node {
                0 => 5,
                1 => -5,
                2 => 10,
                3 => 10,
                4 => 5,
                5 => -10,
                6 => 7,
                _ => panic!("Invalid Node"),
            }
        }
    }

    impl SearchTree for MockSearchTree {
        type Move = MockMove;
        type Score = i32;
        
        fn evaluate<F>(&mut self, mut on_ongoing: F) -> Option<Self::Score>
        where
            F: FnMut(Self::Move, &mut Self) -> ControlFlow<()> {
            
            if let Some(score) = self.terminal_score() {
                return Some(score);
            }

            for game_move in self.get_moves() {
                self.make_move(game_move);
                let control = on_ongoing(game_move, self);
                self.undo_move(game_move);

                if control.is_break() {
                    return None;
                }
            }

            None
        }
    }

    impl StateKey for MockSearchTree {
        type Key = usize;

        fn key(&self) -> Self::Key {
            self.node
        }
    }

    impl ZeroSumTree for MockSearchTree {
        type ZScore = i32;
    }

    #[test]
    fn test_negamax_chooses_correct_path() {
        let mut game = MockSearchTree { node: 0, history: Vec::new() };
        let (score, best_move) = negamax(&mut game, &mut |_| {});
        
        // P1 should choose Left (leading to node 1 -> node 4 -> score 5)
        assert_eq!(score, 5);
        assert_eq!(best_move, Some(MockMove::Left));
    }

    #[test]
    fn test_negamax_half_pruned_chooses_correct_path() {
        let mut game = MockSearchTree { node: 0, history: Vec::new() };
        let (score, best_move) = negamax_half_pruned(&mut game, &mut |_| {});
        
        // P1 should choose Left (leading to node 1 -> node 4 -> score 5)
        assert_eq!(score, 5);
        assert_eq!(best_move, Some(MockMove::Left));
    }

    #[test]
    fn test_negamax_pruned_chooses_correct_path() {
        let mut game = MockSearchTree { node: 0, history: Vec::new() };
        let (score, best_move) = negamax_pruned(&mut game, &mut |_| {});
        
        // P1 should choose Left (leading to node 1 -> node 4 -> score 5)
        assert_eq!(score, 5);
        assert_eq!(best_move, Some(MockMove::Left));
    }

    #[test]
    fn test_negamax_pruned_trans_chooses_correct_path() {
        let mut game = MockSearchTree { node: 0, history: Vec::new() };
        let (score, best_move) = negamax_pruned_trans0(&mut game, &mut |_| {});
        
        // P1 should choose Left (leading to node 1 -> node 4 -> score 5)
        assert_eq!(score, 5);
        assert_eq!(best_move, Some(MockMove::Left));
    }

    #[test]
    fn test_negamax_pruned_trans_fills_table() {
        let mut game = MockSearchTree { node: 0, history: Vec::new() };
        let mut transposition_table = HashMap::new();
        negamax_pruned_trans(&mut transposition_table, &mut game, &mut |_| {});
        
        // dbg!(&transposition_table);
        for x in 1..=2_usize {
            assert_eq!(*transposition_table.get(&x).unwrap(), MockSearchTree::optimal_score(x));
        }

        let mut game = MockSearchTree { node: 0, history: Vec::new() };
        let mut visited = 0;
        let (score, best_move) = negamax_pruned_trans(&mut transposition_table, &mut game, 
            &mut |m| {
                match m {
                    MetricEvent::NodeVisited => visited += 1,
                    MetricEvent::PruningTriggered => (),
                }
            }
        );

        // P1 should choose Left (leading to node 1 -> node 4 -> score 5)
        assert_eq!(score, 5);
        assert_eq!(best_move, Some(MockMove::Left));

        // Transposition table should cache game, only visit root
        assert_eq!(visited, 1);
    }
}

