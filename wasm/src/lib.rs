use wasm_bindgen::prelude::*;
use tic_tac_toe::{Board, Cell};

#[wasm_bindgen]
pub struct WasmBoard {
    board: Board,
}

#[wasm_bindgen]
impl WasmBoard {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmBoard {
        WasmBoard { board: Board::new() }
    }

    pub fn reset(&mut self) {
        self.board = Board::new();
    }

    pub fn get_cells(&self) -> Vec<u8> {
        self.board
            .cells
            .iter()
            .map(|c| match c {
                Cell::Empty => 0u8,
                Cell::X => 1u8,
                Cell::O => 2u8,
            })
            .collect()
    }

    pub fn make_move(&mut self, index: usize, player: u8) -> bool {
        if index >= 9 || self.board.cells[index] != Cell::Empty {
            return false;
        }
        self.board.cells[index] = match player {
            1 => Cell::X,
            2 => Cell::O,
            _ => return false,
        };
        true
    }

    pub fn best_move(&self, player: u8) -> Option<usize> {
        let cell = match player {
            1 => Cell::X,
            2 => Cell::O,
            _ => return None,
        };
        self.board.best_move(cell)
    }

    pub fn check_winner(&self) -> Option<u8> {
        self.board
            .check_winner()
            .map(|c| match c {
                Cell::X => 1u8,
                Cell::O => 2u8,
                Cell::Empty => 0u8,
            })
    }

    pub fn winning_line(&self) -> Option<Box<[u32]>> {
        self.board
            .winning_line()
            .map(|line| line.iter().map(|&i| i as u32).collect::<Vec<u32>>().into_boxed_slice())
    }
}
