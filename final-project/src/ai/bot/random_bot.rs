use crate::game::board::Board;
use crate::game::r#move::Move;
use crate::game::player::Player;
use rand::rngs::ThreadRng;
use rand::seq::IndexedRandom;

pub struct RandomBot {
    seed: ThreadRng,
}

impl RandomBot {
    pub(crate) fn new() -> Self {
        RandomBot { seed: rand::rng() }
    }
}

impl Player for RandomBot {
    fn get_move(&mut self, board: &Board, is_white: bool) -> Move {
        let moves = board.get_available_candidate_moves(is_white);
        loop {
            let mv = *moves.choose(&mut self.seed).unwrap();
            match mv {
                Move::Pawn { .. } => {
                    return mv;
                }
                Move::Wall { .. } => {
                    let mut test = *board;
                    if !test.make_move(is_white, mv) {
                        continue;
                    }
                    return mv;
                }
            }
        }
    }
}
