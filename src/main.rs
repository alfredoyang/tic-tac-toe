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

    fn best_move(&self, player: Cell) -> Option<usize> {
        let opponent = match player {
            Cell::X => Cell::O,
            Cell::O => Cell::X,
            _ => return None,
        };

        // Step 1: Check for winning move
        for &[a, b, c] in WIN_CONDITIONS.iter() {
            let cells = [self.cells[a], self.cells[b], self.cells[c]];
            let player_count = cells.iter().filter(|&&c| c == player).count();
            let empty_count = cells.iter().filter(|&&c| c == Cell::Empty).count();
            if player_count == 2 && empty_count == 1 {
                if self.cells[a] == Cell::Empty {
                    return Some(a);
                } else if self.cells[b] == Cell::Empty {
                    return Some(b);
                } else if self.cells[c] == Cell::Empty {
                    return Some(c);
                }
            }
        }

        // Step 2: Block opponent's winning move
        for &[a, b, c] in WIN_CONDITIONS.iter() {
            let cells = [self.cells[a], self.cells[b], self.cells[c]];
            let opponent_count = cells.iter().filter(|&&c| c == opponent).count();
            let empty_count = cells.iter().filter(|&&c| c == Cell::Empty).count();
            if opponent_count == 2 && empty_count == 1 {
                if self.cells[a] == Cell::Empty {
                    return Some(a);
                } else if self.cells[b] == Cell::Empty {
                    return Some(b);
                } else if self.cells[c] == Cell::Empty {
                    return Some(c);
                }
            }
        }

        // Step 3: Maximize potential winning lines (functional style)
        (0..9)
            .filter(|&i| self.cells[i] == Cell::Empty)
            .map(|i| {
                let score = WIN_CONDITIONS
                    .iter()
                    .filter(|&&it| it.contains(&i))
                    .map(|&pos_move| {
                        let player_count = pos_move
                            .iter()
                            .filter(|&&c| self.cells[c] == player)
                            .count();
                        let opponent_count = pos_move
                            .iter()
                            .filter(|&&c| self.cells[c] == opponent)
                            .count();
                        let empty_count = pos_move
                            .iter()
                            .filter(|&&c| self.cells[c] == Cell::Empty)
                            .count();
                        if opponent_count == 0 && empty_count > 0 {
                            // when there is a player count, there are more possibilities to win.
                            player_count + 1
                        } else {
                            0
                        }
                    })
                    .sum::<usize>();
                (i, score)
            })
            .max_by_key(|&(_, score)| score)
            .map(|(i, _)| i)
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

        // Case 1: Empty board, should pick center
        assert_eq!(
            board.best_move(Cell::X),
            Some(4),
            "Empty board should pick center"
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
}
