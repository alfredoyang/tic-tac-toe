use std::fmt;
#[cfg(not(target_arch = "wasm32"))]
use std::io;
#[cfg(not(target_arch = "wasm32"))]
use std::io::Write;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Cell {
    Empty,
    X,
    O,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::X => write!(f, "X"),
            Cell::O => write!(f, "O"),
        }
    }
}

const WIN_CONDITIONS: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

/// The board layout is:
///   [0][1][2]
///   [3][4][5]
///   [6][7][8]
#[derive(Clone)]
pub struct Board {
    pub cells: [Cell; 9],
}

impl Board {
    pub fn new() -> Board {
        Board { cells: [Cell::Empty; 9] }
    }

    pub fn print(&self) {
        for row in 0..3 {
            for col in 0..3 {
                print!("{}", self.cells[row * 3 + col]);
                if col < 2 { print!(" "); }
            }
            println!();
        }
    }

    pub fn check_winner(&self) -> Option<Cell> {
        for &[a, b, c] in WIN_CONDITIONS.iter() {
            if self.cells[a] != Cell::Empty &&
               self.cells[a] == self.cells[b] &&
               self.cells[b] == self.cells[c] {
                return Some(self.cells[a]);
            }
        }
        None
    }

    pub fn is_full(&self) -> bool {
        !self.cells.contains(&Cell::Empty)
    }

    fn is_block_move(&self, index: usize, player: Cell) -> bool {
        let opponent = match player {
            Cell::X => Cell::O,
            Cell::O => Cell::X,
            _ => return false,
        };

        for &[a, b, c] in WIN_CONDITIONS.iter() {
            let indices = [a, b, c];
            if indices.contains(&index) {
                let cells = [self.cells[a], self.cells[b], self.cells[c]];
                let opponent_count = cells.iter().filter(|&&c| c == opponent).count();
                let empty_count = cells.iter().filter(|&&c| c == Cell::Empty).count();
                if opponent_count == 2 && empty_count == 1 {
                    return true;
                }
            }
        }
        false
    }

    pub fn best_move(&self, player: Cell) -> Option<usize> {
        if player != Cell::X && player != Cell::O {
            return None;
        }

        let opponent = match player {
            Cell::X => Cell::O,
            Cell::O => Cell::X,
            _ => unreachable!(),
        };

        let mut best_score = i32::MIN;
        let mut best_move = None;
        for i in 0..9 {
            if self.cells[i] == Cell::Empty {
                let mut board = self.clone();
                board.cells[i] = player;
                let score = board.minimax(opponent, player);
                if score > best_score {
                    best_score = score;
                    best_move = Some(i);
                } else if score == best_score {
                    let block_curr = self.is_block_move(i, player);
                    if let Some(bm) = best_move {
                        if block_curr && !self.is_block_move(bm, player) {
                            best_move = Some(i);
                        }
                    }
                }
            }
        }
        best_move
    }

    fn minimax(&self, turn: Cell, maximizing_player: Cell) -> i32 {
        if let Some(winner) = self.check_winner() {
            return if winner == maximizing_player { 1 } else { -1 };
        }
        if self.is_full() {
            return 0;
        }

        let opponent = match turn {
            Cell::X => Cell::O,
            Cell::O => Cell::X,
            _ => unreachable!(),
        };

        if turn == maximizing_player {
            let mut best = i32::MIN;
            for i in 0..9 {
                if self.cells[i] == Cell::Empty {
                    let mut board = self.clone();
                    board.cells[i] = turn;
                    let score = board.minimax(opponent, maximizing_player);
                    best = best.max(score);
                }
            }
            best
        } else {
            let mut best = i32::MAX;
            for i in 0..9 {
                if self.cells[i] == Cell::Empty {
                    let mut board = self.clone();
                    board.cells[i] = turn;
                    let score = board.minimax(opponent, maximizing_player);
                    best = best.min(score);
                }
            }
            best
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_player_move(&self) -> Option<usize> {
        println!("Enter a number (0-8) to make a move:");
        for row in 0..3 {
            for col in 0..3 {
                let idx = row * 3 + col;
                if self.cells[idx] == Cell::Empty {
                    print!("[{}]", idx);
                } else {
                    print!("[{}]", self.cells[idx]);
                }
                if col < 2 { print!(" "); }
            }
            println!();
        }

        print!("Your move: ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse::<usize>() {
            Ok(idx) if idx < 9 && self.cells[idx] == Cell::Empty => Some(idx),
            _ => {
                println!("Invalid move! Please enter a number between 0 and 8 for an empty cell.");
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_best_move() {
        let mut board = Board::new();
        let first = board.best_move(Cell::X);
        assert!(
            first == Some(4)
                || first == Some(0)
                || first == Some(2)
                || first == Some(6)
                || first == Some(8),
            "Empty board should pick a corner or center, got {:?}",
            first
        );

        board.cells = [Cell::Empty; 9];
        board.cells[0] = Cell::X;
        board.cells[1] = Cell::X;
        assert_eq!(board.best_move(Cell::X), Some(2));

        board.cells = [Cell::Empty; 9];
        board.cells[3] = Cell::O;
        board.cells[4] = Cell::O;
        assert_eq!(board.best_move(Cell::X), Some(5));

        board.cells = [
            Cell::X, Cell::O, Cell::X,
            Cell::O, Cell::X, Cell::O,
            Cell::X, Cell::O, Cell::X,
        ];
        assert_eq!(board.best_move(Cell::X), None);

        board.cells = [Cell::Empty; 9];
        board.cells[4] = Cell::O;
        let next_move = board.best_move(Cell::X);
        assert!(
            next_move == Some(0)
                || next_move == Some(2)
                || next_move == Some(6)
                || next_move == Some(8)
        );

        board.cells = [Cell::Empty; 9];
        assert_eq!(board.best_move(Cell::Empty), None);
    }

    #[test]
    fn test_check_winner() {
        let mut board = Board::new();
        board.cells[0] = Cell::X;
        board.cells[1] = Cell::X;
        board.cells[2] = Cell::X;
        assert_eq!(board.check_winner(), Some(Cell::X));

        board.cells = [
            Cell::X, Cell::O, Cell::X,
            Cell::X, Cell::O, Cell::O,
            Cell::O, Cell::X, Cell::X,
        ];
        assert_eq!(board.check_winner(), None);
    }
}
