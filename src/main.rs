use std::fmt;
use std::io;
use std::io::Write;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Cell {
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
    [0, 1, 2], // Row 1
    [3, 4, 5], // Row 2
    [6, 7, 8], // Row 3
    [0, 3, 6], // Column 1
    [1, 4, 7], // Column 2
    [2, 5, 8], // Column 3
    [0, 4, 8], // Diagonal top-left to bottom-right
    [2, 4, 6], // Diagonal top-right to bottom-left
];

// The board layout is:
//   [0][1][2]
//   [3][4][5]
//   [6][7][8]
#[derive(Clone)]
struct Board {
    cells: [Cell; 9],
}

impl Board {
    fn new() -> Board {
        Board {
            cells: [Cell::Empty; 9],
        }
    }

    fn print(&self) {
        for row in 0..3 {
            for col in 0..3 {
                print!("{}", self.cells[row * 3 + col]);
                if col < 2 {
                    print!(" ");
                }
            }
            println!();
        }
    }

    fn check_winner(&self) -> Option<Cell> {
        // Check each win condition
        for &[a, b, c] in WIN_CONDITIONS.iter() {
            if self.cells[a] != Cell::Empty
                && self.cells[a] == self.cells[b]
                && self.cells[b] == self.cells[c]
            {
                return Some(self.cells[a]);
            }
        }

        None // No winner found
    }

    fn is_full(&self) -> bool {
        !self.cells.contains(&Cell::Empty)
    }

    /// Return `true` if placing a piece at `index` would stop the opponent
    /// from completing a three-in-a-row on their next turn.
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

    /// Compute the best move for `player` using the minimax algorithm.
    /// When scores are equal, prefer moves that immediately block threats.
    fn best_move(&self, player: Cell) -> Option<usize> {
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

    /// Recursively evaluate the board using the minimax algorithm.
    /// `turn` indicates whose move is being simulated and
    /// `maximizing_player` is the player we are optimizing for.
    /// Returns 1 for a win, -1 for a loss and 0 for a draw.
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

    fn get_player_move(&self) -> Option<usize> {
        // Print the board layout with indices
        println!("Enter a number (0-8) to make a move:");
        for row in 0..3 {
            for col in 0..3 {
                let idx = row * 3 + col;
                if self.cells[idx] == Cell::Empty {
                    print!("[{}]", idx);
                } else {
                    print!("[{}]", self.cells[idx]);
                }
                if col < 2 {
                    print!(" ");
                }
            }
            println!();
        }

        // Prompt for input
        print!("Your move: ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        // Parse and validate input
        match input.trim().parse::<usize>() {
            Ok(idx) if idx < 9 && self.cells[idx] == Cell::Empty => Some(idx),
            _ => {
                println!("Invalid move! Please enter a number between 0 and 8 for an empty cell.");
                None
            }
        }
    }
}

fn main() {
    let mut board = Board::new();
    println!("Welcome to Tic-Tac-Toe!");
    println!("Do you want to play first (as X)? (y/n)");
    print!("Choice: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    // Set player and computer roles
    let (player, _computer) = if input.trim().to_lowercase().starts_with('y') {
        (Cell::X, Cell::O)
    } else {
        (Cell::O, Cell::X)
    };

    // Game loop
    loop {
        println!("\nCurrent board:");
        board.print();

        // Get Cell::X move
        if player == Cell::X {
            loop {
                match board.get_player_move() {
                    Some(idx) => {
                        board.cells[idx] = Cell::X;
                        break;
                    }
                    None => continue,
                }
            }
        } else if let Some(idx) = board.best_move(Cell::X) {
            println!("Computer plays X at {}:", idx);
            board.cells[idx] = Cell::X;
        }

        // Check for result after X's move
        if let Some(winner) = board.check_winner() {
            println!("\nFinal board:");
            board.print();
            println!(
                "{}",
                if winner == player {
                    "You win!"
                } else {
                    "You lose!"
                }
            );
            break;
        }
        if board.is_full() {
            println!("\nFinal board:");
            board.print();
            println!("Draw!");
            break;
        }

        println!("\nCurrent board:");
        board.print();

        // Get Cell::O move
        if player == Cell::O {
            loop {
                match board.get_player_move() {
                    Some(idx) => {
                        board.cells[idx] = Cell::O;
                        break;
                    }
                    None => continue,
                }
            }
        } else if let Some(idx) = board.best_move(Cell::O) {
            println!("Computer plays O at {}:", idx);
            board.cells[idx] = Cell::O;
        }

        // Check for result after O's move
        if let Some(winner) = board.check_winner() {
            println!("\nFinal board:");
            board.print();
            println!(
                "{}",
                if winner == player {
                    "You win!"
                } else {
                    "You lose!"
                }
            );
            break;
        }
        if board.is_full() {
            println!("\nFinal board:");
            board.print();
            println!("Draw!");
            break;
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

        // Case 1: Empty board, should pick a corner or center
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

        // Case 2: X can win by placing at index 2 (row [0,1,2])
        board.cells = [Cell::Empty; 9]; // Reset board
        board.cells[0] = Cell::X;
        board.cells[1] = Cell::X;
        assert_eq!(
            board.best_move(Cell::X),
            Some(2),
            "X should win by placing at 2"
        );

        // Case 3: X must block O at index 5 (row [3,4,5])
        board.cells = [Cell::Empty; 9];
        board.cells[3] = Cell::O;
        board.cells[4] = Cell::O;
        assert_eq!(board.best_move(Cell::X), Some(5), "X should block O at 5");

        // Case 4: Full board, no move possible
        board.cells = [
            Cell::X, Cell::O, Cell::X,
            Cell::O, Cell::X, Cell::O,
            Cell::X, Cell::O, Cell::X,
        ];
        assert_eq!(
            board.best_move(Cell::X),
            None,
            "No move possible on full board"
        );

        // Case 5: X maximizes potential (e.g., after O takes center)
        board.cells = [Cell::Empty; 9];
        board.cells[4] = Cell::O;
        let next_move = board.best_move(Cell::X);
        assert!(
            next_move == Some(0) ||
            next_move == Some(2) ||
            next_move == Some(6) ||
            next_move == Some(8),
            "X should pick a corner (0, 2, 6, or 8) after O takes center, got {:?}", next_move
        );

        // Case 6: Invalid player
        board.cells = [Cell::Empty; 9];
        assert_eq!(
            board.best_move(Cell::Empty),
            None,
            "Invalid player returns None"
        );
    }
    #[test]
    fn test_check_winner() {
        // Winning scenario: X fills the top row
        let mut board = Board::new();
        board.cells[0] = Cell::X;
        board.cells[1] = Cell::X;
        board.cells[2] = Cell::X;
        assert_eq!(board.check_winner(), Some(Cell::X));

        // Draw scenario: board is full without a winner
        board.cells = [
            Cell::X, Cell::O, Cell::X,
            Cell::X, Cell::O, Cell::O,
            Cell::O, Cell::X, Cell::X,
        ];
        assert_eq!(board.check_winner(), None);
    }
}
