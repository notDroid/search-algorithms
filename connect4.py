import numpy as np


class Connect4Board:
  def __init__(self, seq, dims=(6, 7)):
    # constants
    self.initial_seq = seq
    self.dims = dims
    self.max_score = (dims[0] * dims[1])//2 + 1

    # Create mutable state: board, cols, last_move, turn, n, terminal
    self.board = np.zeros(dims, dtype=np.int8)
    self.col_lengths = np.zeros(dims[1], dtype=np.int8)
    for i, col in enumerate(seq):
      col -= 1
      row = self.col_lengths[col]
      if row == dims[0]:
        raise ValueError(f'Column {col} is full')
      
      self.col_lengths[col] += 1
      self.board[row, col] = 1 + (i & 1)
    
    self.seq = list(seq)
    self.last_move = seq[-1]-1
    self.n = len(seq)
    self.turn = self.n & 1

    self._tc = False
    self.terminal = False
    self.terminal_mode = ""

  def get_next_moves(self):
    return np.where(self.board[-1] == 0)[0].astype(np.int8)

  def is_terminal(self):
    if self._tc: # if false, we need to check
      return self.terminal
    self._tc = True

    if self._is_connected():
      self.terminal = True
      self.terminal_mode = "connected"
      return True
    if self.n == self.dims[0] * self.dims[1]:
      self.terminal = True
      self.terminal_mode = "draw"
      return True
    self.terminal = False
    self.terminal_mode = ""
    return False

  def _is_connected(self):
    player = 1 + ((self.turn - 1) & 1)
    row = self.col_lengths[self.last_move] - 1
    col = self.last_move

    def scan(delta_i, delta_j):
      length = 0
      i = delta_i
      j = delta_j
      while (
          row+i < self.dims[0] and
          row+i >= 0           and
          col+j < self.dims[1] and
          col+j >= 0           and
          (self.board[row+i, col+j] == player)
      ):
        i += delta_i
        j += delta_j
        length += 1
      return length
    
    # Horizontal
    length = scan(0, 1) + scan(0, -1) + 1
    if length >= 4:
      return True
    # Vertical
    length = scan(-1, 0) + 1
    if length >= 4:
      return True
    # Up Right Diagonal
    length = scan(1, 1) + scan(-1, -1) + 1
    if length >= 4:
      return True
    # Down Right Diagonal
    length = scan(-1, 1) + scan(1, -1) + 1
    return length >= 4
  
  def get_score(self):
    if self.terminal_mode == "connected": 
       return ((self.n>>1) + self.turn) - self.max_score
    if self.terminal_mode == "draw":
      return 0
    raise ValueError(f"Terminal mode is {self.terminal_mode}")

  def make_move(self, col):
    row = self.col_lengths[col]
    if row == self.dims[0]:
      raise ValueError(f'Column {col} is full')

    self.col_lengths[col] += 1
    self.board[row, col] = 1 + self.turn

    self.seq.append(col + 1)
    self.last_move = col
    self.n += 1
    self.turn = self.n & 1
    self._tc = False
    # self.is_terminal()

  def undo_move(self, col):
    row = self.col_lengths[col] - 1

    self.col_lengths[col] -= 1
    self.board[row, col] = 0

    self.seq = self.seq[:-1]
    self.last_move = int(self.seq[-1])-1
    self.n -= 1
    self.turn = self.n & 1
    
  def __str__(self):
    s = ""
    for row in reversed(self.board):
      for c in row:
        if c == 0:
          s += '_ '
        elif c == 1:
          s += 'R '
        else:
          s += 'Y '
      s += '\n'
    return s