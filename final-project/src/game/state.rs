use crate::game::board::Board;
use crate::game::r#move::Move;

pub enum GameResult {
    InProgress,
    WhiteWins,
    BlackWins,
}

pub fn check_result(board: &Board) -> GameResult {
    if board.white_pos() >= 72 { GameResult::WhiteWins }
    else if board.black_pos() <= 8 { GameResult::BlackWins }
    else { GameResult::InProgress }
}

pub trait Player {
    fn get_move(&mut self, board: &Board, is_white: bool) -> Move;
}
