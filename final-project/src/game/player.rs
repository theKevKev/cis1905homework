use crate::game::board::Board;
use crate::game::r#move::Move;

pub trait Player {
    fn get_move(&mut self, board: &Board, is_white: bool) -> Move;
}
