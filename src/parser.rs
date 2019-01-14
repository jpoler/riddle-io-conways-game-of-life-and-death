use std::error::Error as StdError;
use std::io;
use types::{Action, Coordinate, Message, Move, Setting, Square, Update};

#[derive(Debug)]
pub struct ParserError {
    pub message: String,
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "parser error: {}", self.message)
    }
}

fn unknown_option(option: &str, line: &str, line_no: usize) -> ParserError {
    ParserError {
        message: format!(
            "Unknown option {} on line {} ({})",
            option,
            line_no,
            line.trim()
        ),
    }
}

impl StdError for ParserError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl From<io::Error> for ParserError {
    fn from(e: io::Error) -> ParserError {
        ParserError {
            message: format!("io error: {}", e.description()),
        }
    }
}

impl From<std::num::ParseIntError> for ParserError {
    fn from(e: std::num::ParseIntError) -> ParserError {
        ParserError {
            message: format!("parse int error: {}", e.description()),
        }
    }
}

pub struct Parser<T> {
    buf: T,
    line_no: usize,
}

impl<'a, T: std::io::BufRead> Parser<T> {
    pub fn new(buf: T) -> Parser<T> {
        Parser { buf, line_no: 0 }
    }

    fn parse_line(&mut self) -> Result<Message, ParserError> {
        let mut line = String::new();
        self.buf.read_line(&mut line)?;
        let mut it = line.split_whitespace();
        let first = it.next().ok_or(ParserError {
            message: format!("missing token on line: {}", self.line_no),
        });
        self.line_no += 1;
        let mut rest: Vec<&str> = it.collect();

        let msg = match first? {
            "settings" => Message::Setting(self.parse_settings(&mut rest, &line)?),
            "update" => Message::Update(self.parse_update(&mut rest, &line)?),
            "action" => Message::Action(self.parse_action(&mut rest, &line)?),
            e @ _ => return Err(unknown_option(e, &line, self.line_no)),
        };

        Ok(msg)
    }

    fn parse_settings(
        &mut self,
        tokens: &mut Vec<&str>,
        line: &str,
    ) -> Result<Setting, ParserError> {
        if tokens.len() < 2 {
            return Err(ParserError {
                message: format!("invalid settings on line: {}", self.line_no),
            });
        }

        let (typ, val) = (tokens[0], tokens[1]);

        let setting = match typ {
            "timebank" => Setting::TimeBank(val.parse::<u64>()?),
            "time_per_move" => Setting::TimePerMove(val.parse::<u64>()?),
            "player_names" => Setting::PlayerNames(
                val.split(",")
                    .map(|s| (*s).to_string())
                    .collect::<Vec<String>>(),
            ),
            "your_bot" => Setting::YourBot(val.to_string()),
            "your_botid" => Setting::YourBotId(val.parse::<u64>()?),
            "field_width" => Setting::FieldWidth(val.parse::<u64>()?),
            "field_height" => Setting::FieldHeight(val.parse::<u64>()?),
            "max_rounds" => Setting::MaxRounds(val.parse::<u64>()?),
            e @ _ => return Err(unknown_option(e, &line, self.line_no)),
        };

        Ok(setting)
    }

    fn parse_update(&mut self, tokens: &mut Vec<&str>, line: &str) -> Result<Update, ParserError> {
        if tokens.len() < 3 {
            return Err(ParserError {
                message: format!("invalid update on line: {}", self.line_no),
            });
        }

        let (first, second, third) = (tokens[0], tokens[1], tokens[2]);

        let update = match (first, second) {
            ("game", "round") => Update::GameRound {
                round: third.parse::<u64>()?,
            },
            ("game", "field") => Update::GameField {
                field: third
                    .split(",")
                    .map(|s| match s {
                        "." => Ok(Square::Empty),
                        "0" => Ok(Square::Player1),
                        "1" => Ok(Square::Player2),
                        e => return Err(unknown_option(e, &line, self.line_no)),
                    }).collect::<Result<Vec<Square>, ParserError>>()?,
            },
            (_, "living_cells") => Update::LivingCells {
                player: first.to_string(),
                cells: third.parse::<u64>()?,
            },
            (_, "move") => Update::Move {
                player: first.to_string(),
                mov: self.parse_move(third, &line)?,
            },
            (e, _) => return Err(unknown_option(e, &line, self.line_no)),
        };

        Ok(update)
    }

    fn parse_action(&mut self, tokens: &mut Vec<&str>, line: &str) -> Result<Action, ParserError> {
        if tokens.len() < 2 {
            return Err(ParserError {
                message: format!("invalid action on line: {}", self.line_no),
            });
        }

        let (typ, val) = (tokens[0], tokens[1]);

        if typ != "move" {
            Err(unknown_option(typ, &line, self.line_no))
        } else {
            Ok(Action::Move {
                time: val.parse::<u64>()?,
            })
        }
    }

    fn parse_move(&mut self, move_token: &str, line: &str) -> Result<Move, ParserError> {
        let mut tokens = move_token.split("_");
        let first = tokens.next().ok_or(ParserError {
            message: format!("missing token at on line '{}' at {}", line, self.line_no),
        })?;

        let coords = tokens
            .map(|c| self.parse_coordinate(c, &line))
            .collect::<Result<Vec<Coordinate>, ParserError>>()?;

        let mov = match first {
            "kill" => {
                if coords.len() != 1 {
                    return Err(ParserError {
                        message: format!(
                            "expected one coordinate on line '{}' at {}",
                            &line, self.line_no
                        ),
                    });
                }

                Move::Kill { loc: coords[0] }
            }

            "birth" => {
                if coords.len() != 3 {
                    return Err(ParserError {
                        message: format!(
                            "expected three coordinates on line '{}' at {}",
                            &line, self.line_no
                        ),
                    });
                }

                Move::Birth {
                    birth: coords[0],
                    sacrifice: [coords[1], coords[2]],
                }
            }
            "pass" => Move::Pass,
            "null" => Move::Null,
            e @ _ => return Err(unknown_option(e, &line, self.line_no)),
        };

        Ok(mov)
    }

    fn parse_coordinate(&mut self, token: &str, line: &str) -> Result<Coordinate, ParserError> {
        let mut it = token.split(",");
        let x_tok = it.next().ok_or(ParserError {
            message: format!(
                "coordinate missing x value on line '{}' at {}",
                &line, self.line_no
            ),
        })?;

        let y_tok = it.next().ok_or(ParserError {
            message: format!(
                "coordinate missing y value on &line '{}' at {}",
                &line, self.line_no
            ),
        })?;

        Ok(Coordinate {
            x: x_tok.parse::<u64>()?,
            y: y_tok.parse::<u64>()?,
        })
    }

    pub fn iter(&'a mut self) -> ParserIter<'a, T> {
        ParserIter { parser: self }
    }
}

pub struct ParserIter<'a, T: 'a> {
    parser: &'a mut Parser<T>,
}

impl<'a, T: std::io::BufRead> Iterator for ParserIter<'a, T> {
    type Item = Result<Message, ParserError>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.parser.parse_line())
    }
}
