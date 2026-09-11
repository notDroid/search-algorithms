use crate::game::{ReversibleGame, ZeroSumGame};

#[derive(Clone, Debug, Copy, PartialEq)]
enum Connect4Player {
    Red,
    Yellow,
}

impl Connect4Player {
    fn from_turn(n: u8) -> Connect4Player {
        match n % 2 {
            0 => Self::Red,
            1 => Self::Yellow,
            _ => unreachable!(),
        }
    }
}

type Connect4Move = usize;

#[derive(Clone, Debug)]
pub struct Connect4 {
    board: [[Option<Connect4Player>; 7]; 6], // used to find terminal positions
    col_lengths: [u8; 7],                    // used for getting last row quickly
    n_moves: u8,                             // used to determine turn
    last_move: Option<Connect4Move>,         // faster terminal check
}

impl Connect4 {
    pub fn from_sequence(seq: &str) -> Result<Self, &'static str> {
        let mut board = Self::new();
        for (i, ch) in seq.chars().enumerate() {
            let col: usize = ch
                .to_digit(10)
                .ok_or("Invalid character")?
                .checked_sub(1)
                .ok_or("Column out of bounds")?
                .try_into()
                .unwrap();

            if col > 6 {
                return Err("Column out of bounds");
            }

            let row: usize = board.col_lengths[col].into();
            if row == 6 {
                return Err("Column full");
            }

            board.board[row][col] = Some(Connect4Player::from_turn(i.try_into().unwrap()));
            board.col_lengths[col] += 1;
            board.n_moves += 1;
            board.last_move = Some(col);
        }
        Ok(board)
    }

    pub fn new() -> Self {
        return Connect4 {
            board: [[None; 7]; 6],
            col_lengths: [0; 7],
            n_moves: 0,
            last_move: None,
        };
    }
}

// We implement ReversibleGame so it can be used with Negamax
impl ReversibleGame for Connect4 {
    type Move = Connect4Move; // A move is just the column index (0-6)

    fn get_moves(&self) -> Vec<Self::Move> {
        self.col_lengths
            .iter()
            .enumerate()
            .filter(|(_, rows)| **rows < 7)
            .map(|(i, _)| i)
            .collect()
    }

    fn make_move(&mut self, game_move: &Self::Move) {
        let col = *game_move;
        if col > 6 {
            panic!("Column out of bounds");
        }

        let row: usize = self.col_lengths[col].into();
        if row == 6 {
            panic!("Column full");
        }

        self.board[row][col] = Some(Connect4Player::from_turn(self.n_moves));
        self.col_lengths[col] += 1;
        self.n_moves += 1;
        self.last_move = Some(*game_move)
    }

    fn undo_move(&mut self, game_move: &Self::Move) {
        let col = *game_move;
        if col > 6 {
            panic!("Column out of bounds");
        }

        self.last_move = None;
        self.n_moves -= 1;
        self.col_lengths[col] -= 1;
        let row: usize = self.col_lengths[col].into();
        self.board[row][col] = None;
    }
}

// We implement ZeroSumGame for scoring
impl ZeroSumGame for Connect4 {
    type Score = i32;

    fn terminal_score(&self) -> Option<Self::Score> {
        let col = self.last_move?;
        let row: usize = (self.col_lengths[col] - 1) as usize;
        let player = self.board[row][col]?;

        let scan = |delta_i, delta_j| {
            let mut length = 0;
            let mut i: isize = (row as isize) + delta_i;
            let mut j: isize = (col as isize) + delta_j;

            while i > 0
                && i < 6
                && j > 0
                && j < 7
                && let Some(p) = self.board[i as usize][j as usize]
                && p == player
            {
                i += delta_i;
                j += delta_j;
                length += 1;
            }

            length
        };

        let mut is_connected = false;

        // horizontal
        if scan(0, 1) + scan(0, -1) + 1 >= 4 {
            is_connected = true;
        }
        // vertical
        else if scan(-1, 0) + 1 >= 4 {
            is_connected = true;
        }
        // up right diagonal
        else if scan(-1, -1) + scan(1, 1) + 1 >= 4 {
            is_connected = true;
        }
        // down left diagonal
        else if scan(-1, 1) + scan(1, -1) + 1 >= 4 {
            is_connected = true;
        }

        if !is_connected {
            return None;
        }

        Some((self.n_moves.div_ceil(2) as i32) - 22)
    }
}
