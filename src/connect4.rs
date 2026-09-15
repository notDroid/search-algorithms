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
) -> impl Iterator<Item = Result<Column, &'static str>> + 'a {
    seq.chars().map(|ch| {
        ch.to_digit(10)
            .ok_or("Invalid character")?
            .checked_sub(1)
            .ok_or("Column out of bounds")?
            .try_into()
            .map_err(|_| "Column out of bounds")
            .and_then(|col| Column::try_from(col))
    })
}

impl Connect4Basic {
    pub fn from_sequence(seq: &str) -> Result<Self, &'static str> {
        let mut game = Self::new();
        for col in seq_iterator(seq) {
            game.make_move(col?);
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

/// Bit board implementation of connect 4 board
#[derive(Default, Clone, Copy)]
pub struct Connect4BitBoard {
    /// Number of turns played
    size: usize,

    /// Binary represenations of the boards as follows:
    /// 
    /// (ROWS+1) x COLUMNS 
    /// 
    /// ```ascii
    /// .  .  .  .  .  .  .
    /// 5 12 19 26 33 40 47
    /// 4 11 18 25 32 39 46
    /// 3 10 17 24 31 38 45
    /// 2  9 16 23 30 37 44
    /// 1  8 15 22 29 36 43
    /// 0  7 14 21 28 35 42 
    /// ```
    /// 
    /// Top row unused.
    /// 
    /// pos and mask together determine the board.
    /// 
    /// pos uses 1 for current player 0 for opposite.
    pos: u64,
    /// 1 for every played square
    mask: u64,
}

impl Connect4BitBoard {
    pub fn from_sequence(seq: &str) -> Result<Self, &'static str> {
        let mut game = Self::default();
        for col in seq_iterator(seq) {
            game = game.insert(col?.index())
        }
        Ok(game)
    }

    /// For pos: 
    /// - Flip every previously played 1->0 and 0->1. 
    ///     - This can be done using pos^mask: 0^0=0, x^1=~x.
    /// - Next played move will be 0 anyways, just need to update mask.
    /// 
    /// For mask:
    /// - mask + bottom_mask -> cascades that column so that 1 is the next row, rest undisturbed.
    /// - OR that to get new mask.
    #[inline]
    fn insert(&self, col: usize) -> Self {
        Self {
            size: self.size + 1,
            pos: self.pos ^ self.mask,
            mask: self.mask | (self.mask + Self::bottom_mask(col)),
        }
    }

    /// move a 1 to the bottom of the corresponding column
    #[inline]
    fn bottom_mask(col: usize) -> u64 {
        1_u64 << (col*(ROWS+1))
    }

    /// Use parallel-scan like operation to check every angle in 2 operations. (Check the game finished the previous turn)
    /// 
    /// Horizontal:
    /// - Shift board 1 to the right then &
    /// - this creates a grid where every tile (excluding the first column) represents whether there is 2 in a row (going leftwards).
    /// - this again but shift twice to the right, then each tile (excluding the first 3 column) represents whether there is a 4 in a row (going leftwards).
    /// - if any tile is non-zero the overall u64 is non-zero and there is a connect4
    /// 
    /// Similiarily for other directions.
    #[inline]
    fn is_connected(&self) -> bool {
        let pos = self.pos ^ self.mask;

        // horizontal 
        let m = pos & (pos >> (ROWS+1));
        if (m & (m >> (2*(ROWS+1)))) != 0 {
            return true;
        }

        // diagonal 1
        let m = pos & (pos >> ROWS);
        if m & (m >> (2*ROWS)) != 0 {
            return true;
        }

        // diagonal 2 
        let m = pos & (pos >> (ROWS+2));
        if m & (m >> (2*(ROWS+2))) != 0 {
            return true;
        }

        // vertical;
        let m = pos & (pos >> 1);
        if m & (m >> 2) != 0 {
            return true;
        }

        return false;
    }

    #[inline]
    fn terminal_score(&self) -> Option<i32> {
        if self.is_connected() {
            Some((self.size.div_ceil(2) as i32) + MIN_SCORE)
        } else if self.size == ROWS*COLUMNS {
            Some(0)
        } else {
            None
        }
    }

    #[inline]
    fn top_mask(col: usize) -> u64 {
        (1 << (COLUMNS-1)) << (col*(ROWS+1))
    }

    #[inline]
    fn column_open(&self, col: usize) -> bool {
        self.mask & Self::top_mask(col) != 0
    }
}

impl SearchTree for Connect4BitBoard {
    type Move = usize;
    type Score = i32;

    fn evaluate<F>(&mut self, mut on_ongoing: F) -> Option<Self::Score>
    where
        F: FnMut(Self::Move, &mut Self) -> ControlFlow<()> {
        if let Some(score) = self.terminal_score() {
            return Some(score);
        }

        for col in 0..COLUMNS {
            if !self.column_open(col) {
                continue;
            }

            let control = on_ongoing(col, &mut self.insert(col));

            if control.is_break() {
                return None;
            }
        }

        None
    }
}

impl ZeroSumTree for Connect4BitBoard {
    type ZScore = i32;
}

#[cfg(test)]
mod tests {

    macro_rules! generate_tests {
        ($GameType:ty) => {
            #[test]
            fn test_horizontal_win() {
                // Red plays col 1, Yellow plays 1, Red 2, Yellow 2, Red 3, Yellow 3, Red 4 -> Win!
                let game = <$GameType>::from_sequence("1122334").unwrap();
                let score = game.terminal_score();
                
                assert!(score.is_some(), "Game should be terminal");
                assert!(score.unwrap() < 0, "Score should be negative from perspective of the losing player");
            }

            #[test]
            fn test_vertical_win() {
                // Red 1, Yellow 2, Red 1, Yellow 2, Red 1, Yellow 2, Red 1 -> Win!
                let game = <$GameType>::from_sequence("1212121").unwrap();
                let score = game.terminal_score();

                assert!(score.is_some());
                assert!(score.unwrap() < 0, "Score should be negative from perspective of the losing player");
            }

            #[test]
            fn test_up_right_diagonal_win() {
                // Builds a diagonal from (col 1, row 0) to (col 4, row 3) for Red
                // Sequence: 1, 2, 2, 3, 3, 4, 3, 4, 4, 1, 4
                let game = <$GameType>::from_sequence("12233434414").unwrap();
                let score = game.terminal_score();

                assert!(score.is_some());
                assert!(score.unwrap() < 0, "Score should be negative from perspective of the losing player");
            }
            
            #[test]
            fn test_draw() {
                // A full board sequence that ends in a draw
                // (Just filling it up without connecting 4)
                let seq = "473441442553113552155666136174332676222777";
                let game = <$GameType>::from_sequence(seq).unwrap();
                let score = game.terminal_score();

                assert!(score.is_some());
                assert!(score.unwrap() == 0, "Score should be 0 when the game is a draw");
            }
        };
    }

    mod basic {
        use super::super::*;
        generate_tests!(Connect4Basic);
    }

    mod bitboard {
        use super::super::*;
        generate_tests!(Connect4BitBoard);
    }
}

