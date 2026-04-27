use std::sync::atomic::{AtomicI32, Ordering};

use rayon::prelude::*;

use crate::ai::evalulator::Evaluator;
use crate::game::board::Board;
use crate::game::r#move::Move;
use crate::game::player::Player;
use crate::game::state::GameResult;

pub struct ParallelAlphaBetaBot<E: Evaluator> {
    depth: u8,
    evaluator: E,
}

impl<E: Evaluator> ParallelAlphaBetaBot<E> {
    pub(crate) fn new(depth: u8, evaluator: E) -> Self {
        ParallelAlphaBetaBot { depth, evaluator }
    }
}

// E: Sync so &self (carrying the evaluator) can be shared across rayon threads.
impl<E: Evaluator + Sync> Player for ParallelAlphaBetaBot<E> {
    fn get_move(&mut self, board: &Board, is_white: bool) -> Move {
        let moves: Vec<Move> = board
            .get_available_candidate_moves(is_white)
            .into_iter()
            .collect();

        let shared_alpha = AtomicI32::new(i32::MIN);
        let shared_beta = AtomicI32::new(i32::MAX);
        let this: &Self = self;

        // Moves are already pawn-first by construction. Find the split index and use
        // slices to avoid copying. Two Rayon syncs (barrier-only, not pool teardown),
        // so wall moves start with the tighter alpha/beta established by the pawn phase.
        let split = moves
            .iter()
            .position(|mv| matches!(mv, Move::Wall { .. }))
            .unwrap_or(moves.len());
        let (pawn_moves, wall_moves) = moves.split_at(split);

        let mut results: Vec<(Move, i32)> = pawn_moves
            .par_iter()
            .filter_map(|&mv| {
                search_root_move(mv, board, is_white, this, &shared_alpha, &shared_beta)
            })
            .collect();

        let wall_results: Vec<(Move, i32)> = wall_moves
            .par_iter()
            .filter_map(|&mv| {
                search_root_move(mv, board, is_white, this, &shared_alpha, &shared_beta)
            })
            .collect();

        results.extend(wall_results);

        let mut best_move: Option<Move> = None;
        let mut best_eval = if is_white { i32::MIN } else { i32::MAX };
        for (mv, eval) in results {
            if best_move.is_none() || eval > best_eval && is_white || eval < best_eval && !is_white
            {
                best_move = Some(mv);
                best_eval = eval;
            }
        }
        best_move.expect("invariant violated: no valid moves found")
    }
}

fn search_root_move<E: Evaluator + Sync>(
    mv: Move,
    board: &Board,
    is_white: bool,
    bot: &ParallelAlphaBetaBot<E>,
    shared_alpha: &AtomicI32,
    shared_beta: &AtomicI32,
) -> Option<(Move, i32)> {
    let alpha = shared_alpha.load(Ordering::Relaxed);
    let beta = shared_beta.load(Ordering::Relaxed);
    if beta <= alpha {
        return None;
    }
    let mut local_board = board.clone();
    if !local_board.make_move(is_white, mv) {
        return None;
    }
    let eval = bot.get_eval(
        &mut local_board,
        !is_white,
        bot.depth - 1,
        alpha,
        beta,
        shared_alpha,
        shared_beta,
    );
    if is_white {
        shared_alpha.fetch_max(eval, Ordering::Relaxed);
    } else {
        shared_beta.fetch_min(eval, Ordering::Relaxed);
    }
    Some((mv, eval))
}

impl<E: Evaluator + Sync> ParallelAlphaBetaBot<E> {
    fn get_eval(
        &self,
        board: &mut Board,
        is_white: bool,
        depth: u8,
        mut alpha: i32,
        mut beta: i32,
        global_alpha: &AtomicI32,
        global_beta: &AtomicI32,
    ) -> i32 {
        alpha = alpha.max(global_alpha.load(Ordering::Relaxed));
        beta = beta.min(global_beta.load(Ordering::Relaxed));
        if beta <= alpha {
            return if is_white { alpha } else { beta };
        }

        if depth == 0 {
            return self.evaluator.eval(board, is_white);
        }
        let result: GameResult = board.check_result();
        match result {
            GameResult::WhiteWins => return i32::MAX,
            GameResult::BlackWins => return i32::MIN,
            _ => {}
        }

        let moves = board.get_available_candidate_moves(is_white);
        let mut best_eval = if is_white { i32::MIN } else { i32::MAX };
        for mv in moves {
            if !board.make_move(is_white, mv) {
                continue;
            }
            let eval = self.get_eval(
                board,
                !is_white,
                depth - 1,
                alpha,
                beta,
                global_alpha,
                global_beta,
            );
            if eval > best_eval && is_white || eval < best_eval && !is_white {
                best_eval = eval;
            }
            board.unmake_move(is_white, mv);

            if is_white {
                alpha = alpha.max(eval);
            } else {
                beta = beta.min(eval);
            }
            if beta <= alpha {
                break;
            }
        }
        best_eval
    }
}
