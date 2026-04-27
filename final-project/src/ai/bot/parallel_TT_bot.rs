use std::sync::atomic::{AtomicI32, Ordering};

use rayon::prelude::*;

use crate::ai::evalulator::Evaluator;
use crate::ai::tt::{NodeType, TT};
use crate::game::board::Board;
use crate::game::r#move::Move;
use crate::game::player::Player;
use crate::game::state::GameResult;

const MAX_EXT: u8 = 10; // total extra plies allowed across a single search path
const PAWN_EXT: u8 = 5; // extension when current player has no walls (pawn-only moves)

pub struct ParallelTTBot<E: Evaluator> {
    depth: u8,
    evaluator: E,
    tt: TT,
}

impl<E: Evaluator> ParallelTTBot<E> {
    /// `tt_size_log2`: TT capacity as 2^n entries. 20 ≈ 16 MB, 22 ≈ 64 MB.
    pub(crate) fn new(depth: u8, evaluator: E, tt_size_log2: u8) -> Self {
        ParallelTTBot { depth, evaluator, tt: TT::new(tt_size_log2) }
    }
}

// E: Sync so &self (evaluator + TT) can be shared across rayon threads.
impl<E: Evaluator + Sync> Player for ParallelTTBot<E> {
    fn get_move(&mut self, board: &Board, is_white: bool) -> Move {
        let moves: Vec<Move> = board
            .get_available_candidate_moves(is_white)
            .into_iter()
            .collect();

        let shared_alpha = AtomicI32::new(i32::MIN);
        let shared_beta = AtomicI32::new(i32::MAX);
        let this: &Self = self;

        // Pawn moves come first by construction; search them first so the TT and
        // shared alpha/beta are primed before the slower wall-move batch starts.
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
            if best_move.is_none()
                || eval > best_eval && is_white
                || eval < best_eval && !is_white
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
    bot: &ParallelTTBot<E>,
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
        MAX_EXT,
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

impl<E: Evaluator + Sync> ParallelTTBot<E> {
    fn get_eval(
        &self,
        board: &mut Board,
        is_white: bool,
        depth: u8,
        mut alpha: i32,
        mut beta: i32,
        ext_budget: u8,
        global_alpha: &AtomicI32,
        global_beta: &AtomicI32,
    ) -> i32 {
        // Poll global bounds from parallel sibling threads.
        alpha = alpha.max(global_alpha.load(Ordering::Relaxed));
        beta = beta.min(global_beta.load(Ordering::Relaxed));
        if beta <= alpha {
            return if is_white { alpha } else { beta };
        }

        // TT lookup. Record original_alpha to classify the node type on store.
        let hash = board.zobrist_hash(is_white);
        let original_alpha = alpha;
        if let Some((score, node_type)) = self.tt.lookup(hash, depth) {
            match node_type {
                NodeType::Exact => return score,
                NodeType::LowerBound => alpha = alpha.max(score),
                NodeType::UpperBound => beta = beta.min(score),
            }
            if beta <= alpha {
                return score;
            }
        }

        let result: GameResult = board.check_result();
        match result {
            // Encode remaining depth in mate scores: prefer faster wins, delay losses.
            // Higher depth = closer to root = win happened sooner = better for winner.
            GameResult::WhiteWins => return i32::MAX - 200 + depth as i32,
            GameResult::BlackWins => return i32::MIN + 200 - depth as i32,
            _ => {}
        }

        // Depth extension: only at leaves (depth == 0), like quiescence search.
        // Applying it at every interior node multiplies work exponentially.
        let (effective_depth, remaining_budget) = if depth == 0 {
            let cur_walls =
                if is_white { board.white_walls_remaining() } else { board.black_walls_remaining() };
            let opp_walls =
                if is_white { board.black_walls_remaining() } else { board.white_walls_remaining() };
            let ext = if cur_walls == 0 && ext_budget >= PAWN_EXT { PAWN_EXT } else { 0 };
            let opp_ext =
                if ext > 0 && opp_walls == 0 && ext_budget >= ext + PAWN_EXT { PAWN_EXT } else { 0 };
            if ext == 0 {
                return self.evaluator.eval(board, is_white);
            }
            (ext + opp_ext, ext_budget - ext - opp_ext)
        } else {
            (depth, ext_budget)
        };

        let moves = board.get_available_candidate_moves(is_white);

        let mut best_eval = if is_white { i32::MIN } else { i32::MAX };
        for mv in &moves {
            if !board.make_move(is_white, *mv) {
                continue;
            }
            let eval = self.get_eval(
                board,
                !is_white,
                effective_depth - 1,
                alpha,
                beta,
                remaining_budget,
                global_alpha,
                global_beta,
            );
            if eval > best_eval && is_white || eval < best_eval && !is_white {
                best_eval = eval;
            }
            board.unmake_move(is_white, *mv);

            if is_white {
                alpha = alpha.max(eval);
            } else {
                beta = beta.min(eval);
            }
            if beta <= alpha {
                break;
            }
        }

        let node_type = if best_eval <= original_alpha {
            NodeType::UpperBound
        } else if best_eval >= beta {
            NodeType::LowerBound
        } else {
            NodeType::Exact
        };
        self.tt.store(hash, best_eval, effective_depth, node_type);

        best_eval
    }
}
