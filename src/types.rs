#[derive(Debug, Default)]
pub struct Settings {
    time_bank: u64,
    time_per_move: u64,
    player_names: Vec<String>,
    your_bot: String,
    your_bot_id: u64,
    field_width: u64,
    field_height: u64,
    max_rounds: u64,
}

#[derive(Debug)]
pub enum Message {
    Empty,
    Setting(Setting),
    Update(Update),
    Action(Action),
    Move(Move),
}

#[derive(Debug)]
pub enum Setting {
    TimeBank(u64),
    TimePerMove(u64),
    PlayerNames(Vec<String>),
    YourBot(String),
    YourBotId(u64),
    FieldWidth(u64),
    FieldHeight(u64),
    MaxRounds(u64),
}

#[derive(Debug)]
pub enum Square {
    Empty,
    Player1,
    Player2,
}

#[derive(Debug)]
pub enum Update {
    GameRound { round: u64 },
    GameField { field: Vec<Square> },
    LivingCells { player: String, cells: u64 },
    Move { player: String, mov: Move },
}

#[derive(Debug)]
pub enum Move {
    Null,
    Kill {
        loc: Coordinate,
    },
    Birth {
        birth: Coordinate,
        sacrifice: [Coordinate; 2],
    },
    Pass,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Coordinate {
    pub x: u64,
    pub y: u64,
}

#[derive(Debug)]
pub enum Action {
    Move { time: u64 },
}
