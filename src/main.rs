use std::io::{self, BufRead};

mod ai;
mod board;

use crate::board::Board;
use crate::board::BoardPosition;
use crate::board::Team;

fn clear_console() {
    print!("{}[2J", 27 as char);
}

fn draw_board(board: &Board) {
    print!("Rust TicTacToe\n");
    print!("  | A | B | C |\n");

    let mut y = 0;
    while y < 3 {
        print!("{} |", y + 1);
        let mut x = 0;
        while x < 3 {
            match board.get_cell(BoardPosition { x, y }) {
                Some(Team::X) => print!(" X |"),
                Some(Team::O) => print!(" O |"),
                None => print!("   |"),
            }
            x = x + 1;
        }
        print!("\n");
        y = y + 1;
    }
}

fn get_move(board: Board) -> Option<BoardPosition> {
    loop {
        let stdin = io::stdin();
        let line = stdin.lock().lines().next().unwrap().unwrap();

        if line == "q" {
            return None;
        }

        if line.len() != 2 {
            println!("Invalid position {line}");
            continue;
        }

        let x_pos = match &line[0..1] {
            "A" | "a" => 0,
            "B" | "b" => 1,
            "C" | "c" => 2,
            _ => u8::MAX,
        };
        let y_pos_res = line[1..2].parse::<u8>();
        if x_pos == u8::MAX || y_pos_res.is_err() {
            println!("Invalid position {line}");
            continue;
        }

        let y_pos = y_pos_res.unwrap();
        if y_pos < 1 || y_pos > 3 {
            println!("Invalid position {line}");
            continue;
        }

        let pos = BoardPosition {
            x: x_pos,
            y: y_pos - 1,
        };
        if !board.is_cell_empty(pos.clone()) {
            println!("Position {line} has already been played");
            continue;
        }

        return Some(pos);
    }
}

fn main() {
    let mut board = Board::new();

    let is_second_player_ai = true; // TODO: Configurable
    let mut is_game_over = false;

    loop {
        clear_console();
        draw_board(&board);

        if !is_game_over {
            if is_second_player_ai && board.current_player == Team::O {
                let mv = ai::get_next_move(board.clone());
                board.set_cell(mv);
            } else {
                println!("Input positions like this \"A2\"");

                let m_opt = get_move(board.clone());
                if m_opt.is_none() {
                    break;
                }
                let mv = m_opt.unwrap();
                board.set_cell(mv);
            }

            if board.is_full() || board.evaluate() {
                is_game_over = true;
            } else {
                board.swap_players();
            }
        } else {
            if board.evaluate() {
                print!("Player ");
                match board.current_player {
                    Team::X => print!("X"),
                    Team::O => print!("O"),
                }
                println!(" wins!");
            } else if board.is_full() {
                println!("Board is full");
            }

            loop {
                let _ = io::stdin();
                // TODO: Allow restart, and maybe a score system?
            }
        }
    }

    println!("Goodbye!");
}
