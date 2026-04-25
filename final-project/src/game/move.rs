use super::board::{Board, PositionIndex};
use arrayvec::ArrayVec;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Move {
    Pawn {
        from: PositionIndex,
        to: PositionIndex,
    },
    Wall {
        corner_idx: u8,
        orientation: Orientation,
    },
}

impl Board {
    // Pawn Move Generators
    #[inline(always)]
    fn add_available_up_moves(
        &self,
        my_idx: PositionIndex,
        opponent_idx: PositionIndex,
        moves: &mut ArrayVec<Move, 133>,
    ) {
        if !self.wall_above(my_idx) {
            if my_idx + 9 != opponent_idx {
                moves.push(Move::Pawn {
                    from: my_idx,
                    to: my_idx + 9,
                });
            } else {
                if !self.wall_above(opponent_idx) {
                    moves.push(Move::Pawn {
                        from: my_idx,
                        to: opponent_idx + 9,
                    });
                } else {
                    if !self.wall_right(opponent_idx) {
                        moves.push(Move::Pawn {
                            from: my_idx,
                            to: opponent_idx + 1,
                        });
                    }
                    if !self.wall_left(opponent_idx) {
                        moves.push(Move::Pawn {
                            from: my_idx,
                            to: opponent_idx - 1,
                        });
                    }
                }
            }
        }
    }

    #[inline(always)]
    fn add_available_down_moves(
        &self,
        my_idx: PositionIndex,
        opponent_idx: PositionIndex,
        moves: &mut ArrayVec<Move, 133>,
    ) {
        if !self.wall_below(my_idx) {
            if my_idx - 9 != opponent_idx {
                moves.push(Move::Pawn {
                    from: my_idx,
                    to: my_idx - 9,
                });
            } else {
                if !self.wall_below(opponent_idx) {
                    moves.push(Move::Pawn {
                        from: my_idx,
                        to: opponent_idx - 9,
                    });
                } else {
                    if !self.wall_right(opponent_idx) {
                        moves.push(Move::Pawn {
                            from: my_idx,
                            to: opponent_idx + 1,
                        });
                    }
                    if !self.wall_left(opponent_idx) {
                        moves.push(Move::Pawn {
                            from: my_idx,
                            to: opponent_idx - 1,
                        });
                    }
                }
            }
        }
    }

    #[inline(always)]
    fn add_available_right_moves(
        &self,
        my_idx: PositionIndex,
        opponent_idx: PositionIndex,
        moves: &mut ArrayVec<Move, 133>,
    ) {
        if !self.wall_right(my_idx) {
            if my_idx + 1 != opponent_idx {
                moves.push(Move::Pawn {
                    from: my_idx,
                    to: my_idx + 1,
                });
            } else {
                if !self.wall_right(opponent_idx) {
                    moves.push(Move::Pawn {
                        from: my_idx,
                        to: opponent_idx + 1,
                    });
                } else {
                    if !self.wall_above(opponent_idx) {
                        moves.push(Move::Pawn {
                            from: my_idx,
                            to: opponent_idx + 9,
                        });
                    }
                    if !self.wall_below(opponent_idx) {
                        moves.push(Move::Pawn {
                            from: my_idx,
                            to: opponent_idx - 9,
                        });
                    }
                }
            }
        }
    }

    #[inline(always)]
    fn add_available_left_moves(
        &self,
        my_idx: PositionIndex,
        opponent_idx: PositionIndex,
        moves: &mut ArrayVec<Move, 133>,
    ) {
        if !self.wall_left(my_idx) {
            if my_idx - 1 != opponent_idx {
                moves.push(Move::Pawn {
                    from: my_idx,
                    to: my_idx - 1,
                });
            } else {
                if !self.wall_left(opponent_idx) {
                    moves.push(Move::Pawn {
                        from: my_idx,
                        to: opponent_idx - 1,
                    });
                } else {
                    if !self.wall_above(opponent_idx) {
                        moves.push(Move::Pawn {
                            from: my_idx,
                            to: opponent_idx + 9,
                        });
                    }
                    if !self.wall_below(opponent_idx) {
                        moves.push(Move::Pawn {
                            from: my_idx,
                            to: opponent_idx - 9,
                        });
                    }
                }
            }
        }
    }

    #[inline(always)]
    fn add_available_pawn_moves(
        &self,
        my_idx: PositionIndex,
        opponent_idx: PositionIndex,
        moves: &mut ArrayVec<Move, 133>,
    ) {
        self.add_available_up_moves(my_idx, opponent_idx, moves);
        self.add_available_down_moves(my_idx, opponent_idx, moves);
        self.add_available_left_moves(my_idx, opponent_idx, moves);
        self.add_available_right_moves(my_idx, opponent_idx, moves);
    }

    #[inline(always)]
    fn add_available_candidate_wall_moves(&self, moves: &mut ArrayVec<Move, 133>) {
        for corner_idx in 0..64u8 {
            if (self.walls.corners & (1u64 << corner_idx)) != 0 {
                continue;
            }

            let square_idx = (corner_idx / 8) * 9 + (corner_idx % 8);

            if ((self.walls.walls_above & (1u128 << square_idx)) == 0)
                && ((self.walls.walls_above & (1u128 << (square_idx + 1))) == 0)
            {
                moves.push(Move::Wall {
                    corner_idx,
                    orientation: Orientation::Horizontal,
                });
            }
            if ((self.walls.walls_right & (1u128 << square_idx)) == 0)
                && ((self.walls.walls_right & (1u128 << (square_idx + 9))) == 0)
            {
                moves.push(Move::Wall {
                    corner_idx,
                    orientation: Orientation::Vertical,
                });
            }
        }
    }

    pub(crate) fn get_available_candidate_moves(&self, is_white_turn: bool) -> ArrayVec<Move, 133> {
        let mut moves = ArrayVec::new();

        let (my_idx, opponent_idx) = if is_white_turn {
            (self.white.position_idx, self.black.position_idx)
        } else {
            (self.black.position_idx, self.white.position_idx)
        };

        self.add_available_pawn_moves(my_idx, opponent_idx, &mut moves);
        if (is_white_turn && (self.white.walls_remaining != 0))
            || (!is_white_turn && (self.black.walls_remaining != 0))
        {
            self.add_available_candidate_wall_moves(&mut moves);
        }

        moves
    }
}

impl Board {
    pub(crate) fn make_move(&mut self, is_white_turn: bool, mv: Move) -> bool {
        match mv {
            Move::Pawn { to, .. } => {
                if is_white_turn {
                    self.white.position_idx = to;
                } else {
                    self.black.position_idx = to;
                }
                true
            }
            Move::Wall {
                corner_idx,
                orientation,
            } => {
                self.walls.corners |= 1u64 << corner_idx;
                let square_idx = (corner_idx / 8) * 9 + (corner_idx % 8);

                match orientation {
                    Orientation::Horizontal => {
                        self.walls.walls_above |= 3u128 << square_idx;
                    }
                    Orientation::Vertical => {
                        self.walls.walls_right |= 513u128 << square_idx;
                    }
                };

                if !self.state_valid() {
                    self.unmake_move(is_white_turn, mv);
                    return false;
                }

                if is_white_turn {
                    self.white.walls_remaining -= 1;
                } else {
                    self.black.walls_remaining -= 1;
                }

                true
            }
        }
    }

    pub(crate) fn unmake_move(&mut self, is_white_turn: bool, mv: Move) -> bool {
        match mv {
            Move::Pawn { from, .. } => {
                if is_white_turn {
                    self.white.position_idx = from;
                } else {
                    self.black.position_idx = from;
                }
            }
            Move::Wall {
                corner_idx,
                orientation,
            } => {
                self.walls.corners &= !(1u64 << corner_idx);
                let square_idx = (corner_idx / 8) * 9 + (corner_idx % 8);
                match orientation {
                    Orientation::Horizontal => {
                        self.walls.walls_above &= !(3u128 << square_idx);
                    }
                    Orientation::Vertical => {
                        self.walls.walls_right &= !(513u128 << square_idx);
                    }
                }
                if is_white_turn {
                    self.white.walls_remaining += 1;
                } else {
                    self.black.walls_remaining += 1;
                }
            }
        };
        true
    }
}
