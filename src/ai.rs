#[path = "./board.rs"]
mod board;

use core::f32;
use std::u8;

use crate::board::Board;
use crate::board::BoardPosition;

pub fn get_next_move(board: Board) -> BoardPosition {
    let mut best_val = f32::NEG_INFINITY;
    let mut best_pos = BoardPosition {
        x: u8::MAX,
        y: u8::MAX,
    };

    let mut y = 0;
    while y < 3 {
        let mut x = 0;
        while x < 3 {
            let pos = BoardPosition { x, y };
            // Skip moves that can not be made
            if board.is_cell_empty(pos.clone()) {
                let mut tmp_board = board.clone();
                tmp_board.set_cell(pos.clone());

                let val = minimax(tmp_board, f32::NEG_INFINITY, f32::INFINITY, false);

                if val > best_val {
                    best_val = val;
                    best_pos = pos;
                }
            }
            x = x + 1;
        }
        y = y + 1;
    }

    assert!(best_val != f32::NEG_INFINITY);
    return best_pos;
}

fn minimax(board: Board, mut alpha: f32, mut beta: f32, is_maximizing: bool) -> f32 {
    if board.evaluate() {
        if is_maximizing {
            return f32::INFINITY;
        } else {
            return f32::NEG_INFINITY;
        }
    }

    if board.is_full() {
        return 0.0;
    }

    if is_maximizing {
        let mut max_val = f32::NEG_INFINITY;

        let mut y = 0;
        while y < 3 {
            let mut x = 0;
            while x < 3 {
                let pos = BoardPosition { x, y };
                // Skip moves that can not be made
                if board.is_cell_empty(pos.clone()) {
                    let mut tmp_board: Board = board.clone();
                    tmp_board.swap_players();
                    tmp_board.set_cell(pos.clone());

                    let val = minimax(tmp_board, alpha, beta, false);
                    max_val = val.max(max_val);

                    alpha = alpha.max(val);
                    if val <= beta {
                        break;
                    }
                }
                x = x + 1;
            }
            y = y + 1;
        }

        return max_val;
    } else {
        let mut min_val = f32::INFINITY;

        let mut y = 0;
        while y < 3 {
            let mut x = 0;
            while x < 3 {
                let pos = BoardPosition { x, y };
                // Skip moves that can not be made
                if board.is_cell_empty(pos.clone()) {
                    let mut tmp_board = board.clone();
                    tmp_board.swap_players();
                    tmp_board.set_cell(pos.clone());

                    let val = minimax(tmp_board, alpha, beta, true);
                    min_val = val.min(min_val);

                    beta = beta.max(val);
                    if val <= alpha {
                        break;
                    }
                }

                x = x + 1;
            }
            y = y + 1;
        }

        return min_val;
    }
}
