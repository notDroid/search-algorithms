use crate::kv::{KVStore, StateKey};
use crate::searchtree::ZeroSumTree;
use crate::transposition_table::KVWithReplacement;
use foldhash::fast::FixedState;
use std::collections::HashMap;
use std::ops::ControlFlow;

pub enum MetricEvent {
    NodeVisited,
    PruningTriggered,
    MovePlayed,
}

pub fn negamax<T: ZeroSumTree, M: FnMut(MetricEvent)>(
    st: &mut T,
    m: &mut M,
) -> (T::Score, Option<T::Move>) {
    m(MetricEvent::NodeVisited);
    let mut best_score = None;
    let mut best_game_move = None;

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
        None => (
            best_score.expect("Non terminal state should have at least one move"),
            best_game_move,
        ),
    }
}

fn negamax_half_pruned_core<T: ZeroSumTree, M: FnMut(MetricEvent)>(
    st: &mut T,
    opp_best_score: Option<T::Score>,
    m: &mut M,
) -> (T::Score, Option<T::Move>) {
    m(MetricEvent::NodeVisited);

    let mut best_score = None;
    let mut best_game_move = None;

    let terminal_score = st.evaluate(|game_move, ct| {
        let (opp_score, _) = negamax_half_pruned_core(ct, best_score, m);

        let score = -opp_score;

        // DON'T TAKE EQUAL SCORES
        if best_score.is_none_or(|best_score| best_score < score) {
            best_score = Some(score);
            best_game_move = Some(game_move);
        }

        if let Some(a) = opp_best_score
            && opp_score <= a
        // SKIP EVEN IF EQUAL
        {
            m(MetricEvent::PruningTriggered);
            best_game_move = None;
            return ControlFlow::Break(());
        }

        ControlFlow::Continue(())
    });

    match terminal_score {
        Some(score) => (score, None),
        None => (
            best_score.expect("Non terminal state should have at least one move"),
            best_game_move,
        ),
    }
}

pub fn negamax_half_pruned<T: ZeroSumTree, M: FnMut(MetricEvent)>(
    st: &mut T,
    m: &mut M,
) -> (T::Score, Option<T::Move>) {
    negamax_half_pruned_core(st, None, m)
}

fn negamax_pruned_core<T: ZeroSumTree, M: FnMut(MetricEvent)>(
    st: &mut T,
    opp_best_score: Option<T::Score>,
    mut prev_best_score: Option<T::Score>,
    m: &mut M,
) -> (T::Score, Option<T::Move>) {
    m(MetricEvent::NodeVisited);

    let mut best_score = None;
    let mut best_game_move = None;

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
            && opp_score <= a
        // SKIP EVEN IF EQUAL
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
        None => (
            best_score.expect("Non terminal state should have at least one move"),
            best_game_move,
        ),
    }
}

pub fn negamax_pruned<T: ZeroSumTree, M: FnMut(MetricEvent)>(
    st: &mut T,
    m: &mut M,
) -> (T::Score, Option<T::Move>) {
    negamax_pruned_core(st, None, None, m)
}

fn negamax_alpha_beta_core<T: ZeroSumTree, M: FnMut(MetricEvent)>(
    st: &mut T,
    mut alpha: Option<T::Score>,
    beta: Option<T::Score>,
    m: &mut M,
) -> (T::Score, Option<T::Move>) {
    m(MetricEvent::NodeVisited);

    let mut best_game_move = None;

    let terminal_score = st.evaluate(|game_move, ct| {
        let (opp_score, _) = negamax_alpha_beta_core(ct, beta.map(|b| -b), alpha.map(|a| -a), m);
        let score = -opp_score;

        // Increment current best score in this line
        if alpha.is_none_or(|a| a < score) {
            alpha = Some(score);
            best_game_move = Some(game_move);
        }

        // Prune if we exceed the opponents best move, they would play a different line
        if let Some(beta) = beta
            && score >= beta
        {
            m(MetricEvent::PruningTriggered);
            best_game_move = None;
            return ControlFlow::Break(());
        }

        ControlFlow::Continue(())
    });

    match terminal_score {
        Some(score) => (score, None),
        None => (
            alpha.expect("Non terminal state should have at least one move"),
            best_game_move,
        ),
    }
}

pub fn negamax_alpha_beta<T: ZeroSumTree, M: FnMut(MetricEvent)>(
    st: &mut T,
    m: &mut M,
) -> (T::Score, Option<T::Move>) {
    negamax_alpha_beta_core(st, None, None, m)
}

fn negamax_pruned_trans_lower_core<T, M, KV>(
    transposition_table: &mut KV,
    st: &mut T,
    opp_best_score: Option<T::Score>,
    mut prev_best_score: Option<T::Score>,
    m: &mut M,
) -> (T::Score, Option<T::Move>)
where
    T: ZeroSumTree + StateKey,
    M: FnMut(MetricEvent),
    KV: KVStore<K = T::Key, V = T::Score>,
{
    m(MetricEvent::NodeVisited);

    let mut best_score = None;
    let mut best_game_move = None;

    // Use lower bound if present to see if we can prune and update the best line score
    if let Some(lower_bound_score) = transposition_table.get(st.key()) {
        // Prune if we exceed the opponents best move, they would play a different line
        if let Some(a) = opp_best_score
            && -lower_bound_score <= a
        {
            m(MetricEvent::PruningTriggered);
            return (lower_bound_score, None);
        }

        // Increment current best score in this line
        if prev_best_score.is_none_or(|prev_best_score| prev_best_score < lower_bound_score) {
            prev_best_score = Some(lower_bound_score);
        }
    }

    // Used to detect upper bounds from not finding any good moves at this node
    let original_prev_best_score = prev_best_score;

    let terminal_score = st.evaluate(|game_move, ct| {
        m(MetricEvent::MovePlayed);
        let (opp_score, _) = negamax_pruned_trans_lower_core(
            transposition_table,
            ct,
            prev_best_score,
            opp_best_score,
            m,
        );

        // if pruned the opp score could be higher, meaning
        // this is an upper bound where actual_score <= score <= prev_best_score.
        let score = -opp_score;

        // Increment current best score at this node
        if best_score.is_none_or(|best_score| best_score < score) {
            best_score = Some(score);
            best_game_move = Some(game_move);
        }

        // Prune if we exceed the opponents best move, they would play a different line
        if let Some(a) = opp_best_score
            && opp_score <= a
        {
            m(MetricEvent::PruningTriggered);
            best_game_move = None;
            return ControlFlow::Break(());
        }

        // Increment current best score in this line
        if prev_best_score.is_none_or(|prev_best_score| prev_best_score < score) {
            prev_best_score = Some(score);
        }

        ControlFlow::Continue(())
    });

    let (score, game_move) = match terminal_score {
        Some(score) => (score, None),
        None => (
            best_score.expect("Non terminal state should have at least one move"),
            best_game_move,
        ),
    };

    // 1. Upperbound:
    //  - If we don't find any scores better than our original best, we will never explore this path,
    //  - we would pass down an upperbound in this case which proves this path isn't worth it.
    // 2. Lowerbound:
    //  - If we pruned early we don't get our actual best score and stop early
    // 3. Exact:
    //  - If we don't prune early and we find a new best for this line
    if original_prev_best_score.is_none_or(|prev_best_score| prev_best_score < score) {
        // New lowerbound must be better than old
        transposition_table.put(st.key(), score);
    }

    (score, game_move)
}

fn negamax_pruned_trans_lower<T, M, KV>(
    transposition_table: &mut KV,
    st: &mut T,
    m: &mut M,
) -> (T::Score, Option<T::Move>)
where
    T: ZeroSumTree + StateKey,
    M: FnMut(MetricEvent),
    KV: KVStore<K = T::Key, V = T::Score>,
{
    negamax_pruned_trans_lower_core(transposition_table, st, None, None, m)
}

pub fn negamax_pruned_trans_lower0<T: ZeroSumTree + StateKey, M: FnMut(MetricEvent)>(
    st: &mut T,
    m: &mut M,
) -> (T::Score, Option<T::Move>) {
    let mut transposition_table = HashMap::with_capacity(65_536);
    negamax_pruned_trans_lower(&mut transposition_table, st, m)
}

pub fn negamax_pruned_trans_lower1<T: ZeroSumTree + StateKey, M: FnMut(MetricEvent)>(
    st: &mut T,
    m: &mut M,
) -> (T::Score, Option<T::Move>) {
    let mut transposition_table = KVWithReplacement::new(65_536);
    negamax_pruned_trans_lower(&mut transposition_table, st, m)
}

pub fn negamax_pruned_trans_lower2<T: ZeroSumTree + StateKey, M: FnMut(MetricEvent)>(
    st: &mut T,
    m: &mut M,
) -> (T::Score, Option<T::Move>) {
    let mut transposition_table =
        KVWithReplacement::new_with_hasher(65_536, FixedState::with_seed(42));
    negamax_pruned_trans_lower(&mut transposition_table, st, m)
}

fn negamax_alpha_beta_trans_lower_core<T: ZeroSumTree + StateKey, M: FnMut(MetricEvent)>(
    transposition_table: &mut HashMap<T::Key, T::Score>,
    st: &mut T,
    mut alpha: Option<T::Score>,
    beta: Option<T::Score>,
    m: &mut M,
) -> (T::Score, Option<T::Move>) {
    m(MetricEvent::NodeVisited);

    // Check transposition table for lowerbound
    if let Some(score) = transposition_table.get(&st.key()) {
        let score = *score;

        // Increment current best score in this line
        if alpha.is_none_or(|a| a < score) {
            alpha = Some(score);
        }

        // Prune if we exceed the opponents best move, they would play a different line
        if let Some(beta) = beta
            && score >= beta
        {
            m(MetricEvent::PruningTriggered);
            return (
                alpha.expect("Unreachable, alpha must have been set earlier if it wasn't already."),
                None,
            );
        }
    }

    let mut best_game_move = None;
    let og_alpha = alpha;

    let terminal_score = st.evaluate(|game_move, ct| {
        m(MetricEvent::MovePlayed);
        let (opp_score, _) = negamax_alpha_beta_trans_lower_core(
            transposition_table,
            ct,
            beta.map(|b| -b),
            alpha.map(|a| -a),
            m,
        );
        let score = -opp_score;

        // Increment current best score in this line
        if alpha.is_none_or(|a| a < score) {
            alpha = Some(score);
            best_game_move = Some(game_move);
        }

        // Prune if we exceed the opponents best move, they would play a different line
        if let Some(beta) = beta
            && score >= beta
        {
            m(MetricEvent::PruningTriggered);
            best_game_move = None;
            return ControlFlow::Break(());
        }

        ControlFlow::Continue(())
    });

    let (score, game_move) = match terminal_score {
        Some(score) => (score, None),
        None => (
            alpha.expect("Non terminal state should have at least one move"),
            best_game_move,
        ),
    };

    // 1. Upperbound:
    //  - If we don't find any scores better than our original best, we will never explore this path,
    //  - we would pass down an upperbound in this case which proves this path isn't worth it.
    //  - The upperbound is og_alpha, which is less accurate than recording the specific best score recieved.
    // 2. Lowerbound:
    //  - If we pruned early we don't get our actual best score and stop early
    // 3. Exact:
    //  - If we don't prune early and we find a new best for this line or just if its terminal
    if og_alpha.is_none_or(|a| a < score) {
        // New lowerbound must be better than old if it exists
        transposition_table.insert(st.key(), score);
    }

    (score, game_move)
}

pub fn negamax_alpha_beta_trans_lower<T: ZeroSumTree + StateKey, M: FnMut(MetricEvent)>(
    transposition_table: &mut HashMap<T::Key, T::Score>,
    st: &mut T,
    m: &mut M,
) -> (T::Score, Option<T::Move>) {
    negamax_alpha_beta_trans_lower_core(transposition_table, st, None, None, m)
}

pub fn negamax_alpha_beta_trans_lower0<T: ZeroSumTree + StateKey, M: FnMut(MetricEvent)>(
    st: &mut T,
    m: &mut M,
) -> (T::Score, Option<T::Move>) {
    let mut transposition_table = HashMap::with_capacity(65_536);
    negamax_alpha_beta_trans_lower(&mut transposition_table, st, m)
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
            F: FnMut(Self::Move, &mut Self) -> ControlFlow<()>,
        {
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
        let mut game = MockSearchTree {
            node: 0,
            history: Vec::new(),
        };
        let (score, best_move) = negamax(&mut game, &mut |_| {});

        // P1 should choose Left (leading to node 1 -> node 4 -> score 5)
        assert_eq!(score, 5);
        assert_eq!(best_move, Some(MockMove::Left));
    }

    #[test]
    fn test_negamax_half_pruned_chooses_correct_path() {
        let mut game = MockSearchTree {
            node: 0,
            history: Vec::new(),
        };
        let (score, best_move) = negamax_half_pruned(&mut game, &mut |_| {});

        // P1 should choose Left (leading to node 1 -> node 4 -> score 5)
        assert_eq!(score, 5);
        assert_eq!(best_move, Some(MockMove::Left));
    }

    #[test]
    fn test_negamax_pruned_chooses_correct_path() {
        let mut game = MockSearchTree {
            node: 0,
            history: Vec::new(),
        };
        let (score, best_move) = negamax_pruned(&mut game, &mut |_| {});

        // P1 should choose Left (leading to node 1 -> node 4 -> score 5)
        assert_eq!(score, 5);
        assert_eq!(best_move, Some(MockMove::Left));
    }

    #[test]
    fn test_negamax_pruned_trans_chooses_correct_path() {
        let mut game = MockSearchTree {
            node: 0,
            history: Vec::new(),
        };
        let (score, best_move) = negamax_pruned_trans_lower0(&mut game, &mut |_| {});

        // P1 should choose Left (leading to node 1 -> node 4 -> score 5)
        assert_eq!(score, 5);
        assert_eq!(best_move, Some(MockMove::Left));
    }

    #[test]
    fn test_alpha_beta_chooses_correct_path() {
        let mut game = MockSearchTree {
            node: 0,
            history: Vec::new(),
        };
        let (score, best_move) = negamax_alpha_beta(&mut game, &mut |_| {});

        // P1 should choose Left (leading to node 1 -> node 4 -> score 5)
        assert_eq!(score, 5);
        assert_eq!(best_move, Some(MockMove::Left));
    }

    #[test]
    fn test_negamax_pruned_trans_fills_table() {
        let mut game = MockSearchTree {
            node: 0,
            history: Vec::new(),
        };
        let mut transposition_table = HashMap::new();
        negamax_pruned_trans_lower(&mut transposition_table, &mut game, &mut |_| {});

        // dbg!(&transposition_table);
        for x in 0..=4_usize {
            assert_eq!(
                *transposition_table.get(&x).unwrap(),
                MockSearchTree::optimal_score(x)
            );
        }

        let mut game = MockSearchTree {
            node: 0,
            history: Vec::new(),
        };
        let (score, _) =
            negamax_pruned_trans_lower(&mut transposition_table, &mut game, &mut |_| {});

        // dbg!(&transposition_table);

        // P1 should choose Left (leading to node 1 -> node 4 -> score 5)
        assert_eq!(score, 5);
        // assert_eq!(best_move, Some(MockMove::Left));
    }

    #[test]
    fn test_alpha_beta_trans_lower_chooses_correct_path() {
        let mut game = MockSearchTree {
            node: 0,
            history: Vec::new(),
        };
        let (score, best_move) = negamax_alpha_beta_trans_lower0(&mut game, &mut |_| {});

        // P1 should choose Left (leading to node 1 -> node 4 -> score 5)
        assert_eq!(score, 5);
        assert_eq!(best_move, Some(MockMove::Left));
    }

    #[test]
    fn test_negamax_alpha_beta_lower_fills_table() {
        let mut game = MockSearchTree {
            node: 0,
            history: Vec::new(),
        };
        let mut transposition_table = HashMap::new();
        negamax_alpha_beta_trans_lower(&mut transposition_table, &mut game, &mut |_| {});

        // dbg!(&transposition_table);
        for x in 0..=4_usize {
            assert_eq!(
                *transposition_table.get(&x).unwrap(),
                MockSearchTree::optimal_score(x)
            );
        }

        let mut game = MockSearchTree {
            node: 0,
            history: Vec::new(),
        };
        let (score, _) =
            negamax_alpha_beta_trans_lower(&mut transposition_table, &mut game, &mut |_| {});

        // dbg!(&transposition_table);

        // P1 should choose Left (leading to node 1 -> node 4 -> score 5)
        assert_eq!(score, 5);
        // assert_eq!(best_move, Some(MockMove::Left));
    }
}
