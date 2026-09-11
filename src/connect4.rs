use crate::game::{ReversibleGame, ZeroSumGame};

const ROWS: usize = 6;
const COLUMNS: usize = 7;

#[derive(Clone, Debug, Copy, PartialEq, Default)]
enum Player {
    #[default]
    Red,
    Yellow,
}

impl Player {
    // fn from_turn(n: usize) -> Player {
    //     match n % 2 {
    //         0 => Self::Red,
    //         1 => Self::Yellow,
    //         _ => unreachable!(),
    //     }
    // }

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
    fn from(col: usize) -> Self {
        if col >= COLUMNS {
            panic!("Column out of bounds");
        }

        Self(col)
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
pub struct Connect4 {
    board: Board,
    turn: Player,
    moves: Vec<(usize, usize, Player)>,
}

impl Connect4 {
    pub fn from_sequence(seq: &str) -> Result<Self, &'static str> {
        let mut game = Self::new();
        for ch in seq.chars() {
            let col: usize = ch
                .to_digit(10)
                .ok_or("Invalid character")?
                .checked_sub(1)
                .ok_or("Column out of bounds")?
                .try_into()
                .map_err(|_| "Column out of bounds")?;
            let col = Column::from(col);

            game.make_move(col);
        }
        Ok(game)
    }

    fn get_moves(&self) -> Vec<Column> {
        self.board.column_lengths()
            .iter()
            .enumerate()
            .filter(|(_, rows)| **rows < 7)
            .map(|(i, _)| Column::from(i))
            .collect()
    }

    fn make_move(&mut self, col: Column) {
        let row = self.board.insert(col, self.turn);
        self.moves.push((row, col.index(), self.turn));
        self.turn = self.turn.switch();
    }

    pub fn new() -> Self {
        Connect4::default()
    }
}

impl ReversibleGame for Connect4 {
    type Move = Column;

    fn get_moves(&self) -> Vec<Self::Move> {
        self.get_moves()
    }

    fn make_move(&mut self, game_move: &Self::Move) {
        self.make_move(*game_move);
    }

    fn undo_move(&mut self, game_move: &Self::Move) {
        self.board.pop(*game_move);
        self.moves.pop();
        self.turn = self.turn.switch();
    }
}

impl ZeroSumGame for Connect4 {
    type Score = i32;

    fn terminal_score(&self) -> Option<Self::Score> {
        let (row, col, player) = *self.moves.last()?;
        if self.board.is_connected(row, col, player) {
            Some((self.board.size().div_ceil(2) as i32) - 22)
        } else {
            None
        }
    }
}
