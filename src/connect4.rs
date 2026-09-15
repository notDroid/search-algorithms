use crate::searchtree::{SearchTree, ZeroSumTree};
use std::ops::ControlFlow;

const ROWS: usize = 6;
const COLUMNS: usize = 7;
const MIN_SCORE: i32 = -22;

#[derive(Clone, Debug, Copy, PartialEq, Default)]
enum Player {
    #[default]
    Red,
    Yellow,
}

impl Player {
    fn switch(self) -> Self {
        match self {
            Player::Red => Player::Yellow,
            Player::Yellow => Player::Red,
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Column(usize);

impl Column {
    fn try_from(col: usize) -> Result<Self, &'static str> {
        if col >= COLUMNS {
            return Err("Column out of bounds");
        }

        Ok(Self(col))
    }

    fn index(&self) -> usize {
        self.0
    }
}

#[derive(Default, Clone, Debug)]
struct Board {
    matrix: [[Option<Player>; COLUMNS]; ROWS],
    col_lengths: [usize; COLUMNS],
    size: usize,
}

impl Board {
    fn insert(&mut self, col: Column, player: Player) -> usize {
        let col = col.index();
        let row = self.col_lengths[col];
        if row >= ROWS {
            panic!("Column is full");
        }

        self.matrix[row][col] = Some(player);
        self.col_lengths[col] += 1;
        self.size += 1;

        row
    }

    fn pop(&mut self, col: Column) {
        let col = col.index();
        let row = self.col_lengths[col]
            .checked_sub(1)
            .expect("Column is empty");
        
        self.col_lengths[col] = row;
        self.matrix[row][col] = None;
        self.size -= 1;
    }

    fn column_lengths(&self) -> &[usize; COLUMNS] {
        &self.col_lengths
    }

    fn size(&self) -> usize {
        self.size
    }

    fn is_connected(&self, row: usize, col: usize, player: Player) -> bool {
        let scan = |delta_i: isize, delta_j: isize| {
            let mut length = 0;
            let mut i = (row as isize) + delta_i;
            let mut j = (col as isize) + delta_j;
            while i >= 0
                && i < ROWS as isize
                && j >= 0
                && j < COLUMNS as isize
                && let Some(p) = self.matrix[i as usize][j as usize]
                && p == player
            {
                i += delta_i;
                j += delta_j;
                length += 1;
            }
            length
        };
        
        // horizontal
        scan(0, 1) + scan(0, -1) + 1 >= 4
        // vertical
        || scan(-1, 0) + 1 >= 4
        // up right diagonal
        || scan(-1, -1) + scan(1, 1) + 1 >= 4
        // down left diagonal
        || scan(-1, 1) + scan(1, -1) + 1 >= 4
    }
}

#[derive(Default, Clone, Debug)]
pub struct Connect4Basic {
    board: Board,
    turn: Player,
    moves: Vec<(usize, usize, Player)>,
}

fn seq_iterator<'a>(
    seq: &'a str,
) -> impl Iterator<Item = Result<usize, &'static str>> + 'a {
    seq.chars().map(|ch| {
        ch.to_digit(10)
            .ok_or("Invalid character")?
            .checked_sub(1)
            .ok_or("Column out of bounds")?
            .try_into()
            .map_err(|_| "Column out of bounds")
    })
}

impl Connect4Basic {
    pub fn from_sequence(seq: &str) -> Result<Self, &'static str> {
        let mut game = Self::new();
        for col in seq_iterator(seq) {
            let col = Column::try_from(col?)?;

            game.make_move(col);
        }
        Ok(game)
    }

    fn get_moves(&self) -> Vec<Column> {
        self.board.column_lengths()
            .iter()
            .enumerate()
            .filter(|(_, rows)| **rows < ROWS)
            .map(|(i, _)| Column::try_from(i).expect("Unreachable"))
            .collect()
    }

    fn make_move(&mut self, col: Column) {
        let row = self.board.insert(col, self.turn);
        self.moves.push((row, col.index(), self.turn));
        self.turn = self.turn.switch();
    }

    fn undo_move(&mut self, col: Column) {
        self.board.pop(col);
        self.moves.pop();
        self.turn = self.turn.switch();
    }

    fn terminal_score(&self) -> Option<i32> {
        let (row, col, player) = *self.moves.last()?;
        if self.board.is_connected(row, col, player) {
            Some((self.board.size().div_ceil(2) as i32) + MIN_SCORE)
        } else if self.board.size() == ROWS*COLUMNS {
            Some(0)
        } else {
            None
        }
    }

    pub fn new() -> Self {
        Self::default()
    }
}

impl SearchTree for Connect4Basic {
    type Move = Column;
    type Score = i32;

    fn evaluate<F>(&mut self, mut on_ongoing: F) -> Option<Self::Score>
    where
        F: FnMut(Self::Move, &mut Self) -> ControlFlow<()>
    {
        if let Some(score) = self.terminal_score() {
            return Some(score);
        }

        for col in self.get_moves() {
            self.make_move(col);
            let control = on_ongoing(col, self);
            self.undo_move(col);

            if control.is_break() {
                return None;
            }
        }

        None
    }
}

impl ZeroSumTree for Connect4Basic {
    type ZScore = i32;
}

// pub struct Connect4BitBoard {
//     size: usize,
//     pos: u64,
//     mask: u64,
// }

// impl Connect4BitBoard {
//     fn make_move(self) -> Self {

//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horizontal_win() {
        // Red plays col 1, Yellow plays 1, Red 2, Yellow 2, Red 3, Yellow 3, Red 4 -> Win!
        let game = Connect4Basic::from_sequence("1122334").unwrap();
        let score = game.terminal_score();
        
        assert!(score.is_some(), "Game should be terminal");
        assert!(score.unwrap() < 0, "Score should be negative from perspective of the losing player");
    }

    #[test]
    fn test_vertical_win() {
        // Red 1, Yellow 2, Red 1, Yellow 2, Red 1, Yellow 2, Red 1 -> Win!
        let game = Connect4Basic::from_sequence("1212121").unwrap();
        let score = game.terminal_score();

        assert!(score.is_some());
        assert!(score.unwrap() < 0, "Score should be negative from perspective of the losing player");
    }

    #[test]
    fn test_up_right_diagonal_win() {
        // Builds a diagonal from (col 1, row 0) to (col 4, row 3) for Red
        // Sequence: 1, 2, 2, 3, 3, 4, 3, 4, 4, 1, 4
        let game = Connect4Basic::from_sequence("12233434414").unwrap();
        let score = game.terminal_score();

        assert!(score.is_some());
        assert!(score.unwrap() < 0, "Score should be negative from perspective of the losing player");
    }
    
    #[test]
    fn test_draw() {
        // A full board sequence that ends in a draw
        // (Just filling it up without connecting 4)
        let seq = "473441442553113552155666136174332676222777";
        let game = Connect4Basic::from_sequence(seq).unwrap();
        let score = game.terminal_score();

        assert!(score.is_some());
        assert!(score.unwrap() == 0, "Score should be 0 when the game is a draw");
    }
}

