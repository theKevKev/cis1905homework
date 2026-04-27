#![allow(dead_code)]

use std::cmp::min;

use crate::game::board::Board;

pub(crate) trait Evaluator {
    fn eval(&self, board: &Board, is_white_turn: bool) -> i32;
}

pub(crate) struct BaseEvaluator {}

impl BaseEvaluator {
    pub(crate) fn new() -> Self {
        BaseEvaluator {}
    }
}

impl Evaluator for BaseEvaluator {
    fn eval(&self, board: &Board, _is_white_turn: bool) -> i32 {
        let (white_dist, black_dist) = board.distances();
        if white_dist == 0 {
            return i32::MAX;
        }
        if black_dist == 0 {
            return i32::MIN;
        }

        (black_dist as i32) - (white_dist as i32)
    }
}

pub(crate) struct SimpleEvaluator {
    dist_weight: u8,
    wall_weight: u8,
}

impl SimpleEvaluator {
    pub(crate) fn new() -> Self {
        SimpleEvaluator {
            dist_weight: 10,
            wall_weight: 1,
        }
    }
}

impl Evaluator for SimpleEvaluator {
    fn eval(&self, board: &Board, _is_white_turn: bool) -> i32 {
        let (white_dist, black_dist) = board.distances();
        if white_dist == 0 {
            return i32::MAX;
        }
        if black_dist == 0 {
            return i32::MIN;
        }

        (self.dist_weight as i32) * ((black_dist as i32) - (white_dist as i32))
            + (self.wall_weight as i32)
                * ((board.white_walls_remaining() as i32) - (board.black_walls_remaining() as i32))
            + if _is_white_turn {
                (self.dist_weight as i32) / 2
            } else {
                -(self.dist_weight as i32) / 2
            }
    }
}

pub(crate) struct UrgentEvaluator {
    dist_weight: u8,
    wall_weight: [u8; 10],
}

impl UrgentEvaluator {
    pub(crate) fn new() -> Self {
        UrgentEvaluator {
            dist_weight: 10,
            wall_weight: [0, 2, 4, 6, 8, 10, 7, 4, 1, 1], // wall weights depending on how far from end
        }
    }
}

impl Evaluator for UrgentEvaluator {
    fn eval(&self, board: &Board, _is_white_turn: bool) -> i32 {
        let (white_dist, black_dist) = board.distances();
        if white_dist == 0 {
            return i32::MAX;
        }
        if black_dist == 0 {
            return i32::MIN;
        }

        let urgency = (min(white_dist, black_dist)) as i32;
        let wall_weight = if urgency > 9 {
            1
        } else {
            self.wall_weight[urgency as usize]
        };

        (self.dist_weight as i32) * ((black_dist as i32) - (white_dist as i32))
            + (wall_weight as i32)
                * ((board.white_walls_remaining() as i32) - (board.black_walls_remaining() as i32))
            + if _is_white_turn {
                (self.dist_weight as i32) / 2
            } else {
                -(self.dist_weight as i32) / 2
            }
    }
}
