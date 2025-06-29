use tic_tac_toe::Board;
use tic_tac_toe::Cell;
use std::io;
use std::io::Write;

fn main() {
    let mut board = Board::new();
    println!("Welcome to Tic-Tac-Toe!");
    println!("Do you want to play first (as X)? (y/n)");
    print!("Choice: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let (player, _computer) = if input.trim().to_lowercase().starts_with('y') {
        (Cell::X, Cell::O)
    } else {
        (Cell::O, Cell::X)
    };

    loop {
        println!("\nCurrent board:");
        board.print();

        if player == Cell::X {
            loop {
                match board.get_player_move() {
                    Some(idx) => { board.cells[idx] = Cell::X; break; }
                    None => continue,
                }
            }
        } else if let Some(idx) = board.best_move(Cell::X) {
            println!("Computer plays X at {}:", idx);
            board.cells[idx] = Cell::X;
        }

        if let Some(winner) = board.check_winner() {
            println!("\nFinal board:");
            board.print();
            println!("{}", if winner == player { "You win!" } else { "You lose!" });
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

        if player == Cell::O {
            loop {
                match board.get_player_move() {
                    Some(idx) => { board.cells[idx] = Cell::O; break; }
                    None => continue,
                }
            }
        } else if let Some(idx) = board.best_move(Cell::O) {
            println!("Computer plays O at {}:", idx);
            board.cells[idx] = Cell::O;
        }

        if let Some(winner) = board.check_winner() {
            println!("\nFinal board:");
            board.print();
            println!("{}", if winner == player { "You win!" } else { "You lose!" });
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
