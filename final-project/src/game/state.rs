use crate::game::board::Board;

#[derive(PartialEq)]
pub enum GameResult {
    InProgress,
    WhiteWins,
    BlackWins,
}

impl Board {
    pub(crate) fn check_result(self: &Self) -> GameResult {
        if self.white_pos() >= 72 {
            GameResult::WhiteWins
        } else if self.black_pos() <= 8 {
            GameResult::BlackWins
        } else {
            GameResult::InProgress
        }
    }
}
