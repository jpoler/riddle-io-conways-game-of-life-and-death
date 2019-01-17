mod board;
mod parser;
mod types;

use std::io::{self, BufRead};
use std::mem;

use board::Board;
use types::{Message, Square, Update};

fn main() {
    let s = io::stdin();
    let br = s.lock();

    let mut parser = parser::Parser::new(br);

    eprintln!("sizeof square: {}", mem::size_of::<Square>());

    let mut sim_board: Option<Board> = None;

    for message in parser.iter() {
        match message {
            Ok(msg) => match msg {
                Message::Action(action) => {
                    // eprintln!("passing: {:?}", action);
                    println!("pass");
                }
                Message::Update(update) => match update {
                    Update::GameField { field } => {
                        // TODO remove this hard coding
                        let game_board = Board::from_state(&field, 16, 18);
                        sim_board = if let Some(mut sim_board) = sim_board {
                            let equal = game_board.state().eq(&sim_board.state());

                            eprintln!("game_board: {:?}", game_board.state());
                            eprintln!("sim_board:  {:?}", sim_board.state());

                            // try writing helper methods to do a diff of counts
                            // for each player, that might give a rough
                            // indicator of how far off you are. See if
                            // different than the living cells update
                            eprintln!("predicted state matches engine state: {}", equal);

                            sim_board.simulate_step();
                            Some(sim_board)
                        } else {
                            let mut sim_board = Board::from_state(&field, 16, 18);
                            sim_board.simulate_step();
                            Some(sim_board)
                        }
                    }
                    Update::GameRound { round } => eprintln!("round: {}", round),
                    _ => {}
                },
                // msg => eprintln!("message: {:?}", msg),
                _ => {}
            },
            Err(err) => eprintln!("error: {:?}", err),
        }
    }
}
