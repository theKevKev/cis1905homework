use crate::ai::evalulator::Evaluator;
use crate::game::board::Board;
use crate::game::r#move::Move;
use crate::game::player::Player;
use crate::game::state::GameResult;

#[cfg(debug_assertions)]
use std::cell::Cell;

#[cfg(debug_assertions)]
thread_local! {
    static NODES: Cell<u64> = Cell::new(0);
}

pub struct AlphaBetaBot<E: Evaluator> {
    depth: u8,
    evaluator: E,
}

impl<E: Evaluator> AlphaBetaBot<E> {
    pub(crate) fn new(depth: u8, evaluator: E) -> Self {
        AlphaBetaBot { depth, evaluator }
    }
}

impl<E: Evaluator> Player for AlphaBetaBot<E> {
    fn get_move(&mut self, board: &Board, is_white: bool) -> Move {
        // we are under the assumption that get_move is not called unless there are moves left to play
        let moves = board.get_available_candidate_moves(is_white);

        #[cfg(debug_assertions)]
        NODES.with(|n| n.set(0));

        let mut best_move: Option<Move> = None;
        let mut best_eval = if is_white { i32::MIN } else { i32::MAX };
        let mut my_board = board.clone();

        let mut alpha = i32::MIN;
        let mut beta = i32::MAX;

        #[cfg(debug_assertions)]
        let mut debug_evals: Vec<(i32, Move)> = Vec::new();
        for mv in moves {
            if !my_board.make_move(is_white, mv) {
                continue;
            }
            let eval = self.get_eval(&mut my_board, !is_white, self.depth - 1, alpha, beta);
            my_board.unmake_move(is_white, mv);

            #[cfg(debug_assertions)]
            debug_evals.push((eval, mv));

            if best_move.is_none() || eval > best_eval && is_white || eval < best_eval && !is_white {
                best_move = Some(mv);
                best_eval = eval;
            }
            if is_white {
                alpha = alpha.max(eval);
            } else {
                beta = beta.min(eval);
            }
            if beta <= alpha {
                break;
            }
        }
        #[cfg(debug_assertions)]
        {
            let (white_dist, black_dist) = board.distances();
            let nodes = NODES.with(|n| n.get());
            debug_evals.sort_by(|a, b| if is_white { b.0.cmp(&a.0) } else { a.0.cmp(&b.0) });
            eprintln!(
                "--- alpha-beta ({}) depth={} white_dist={} black_dist={} nodes={} ---",
                if is_white { "white" } else { "black" },
                self.depth,
                white_dist,
                black_dist,
                nodes,
            );
            for (eval, mv) in &debug_evals {
                let marker = if best_move == Some(*mv) { " <--" } else { "" };
                eprintln!("  {mv}: {eval}{marker}");
            }
        }
        best_move.expect("invariant violated: no valid moves found")
    }
}

impl<E: Evaluator> AlphaBetaBot<E> {
    fn get_eval(
        &self,
        board: &mut Board,
        is_white: bool,
        depth: u8,
        mut alpha: i32,
        mut beta: i32,
    ) -> i32 {
        #[cfg(debug_assertions)]
        NODES.with(|n| n.set(n.get() + 1));

        let result: GameResult = board.check_result();
        match result {
            GameResult::WhiteWins => return i32::MAX - 200 + depth as i32,
            GameResult::BlackWins => return i32::MIN + 200 - depth as i32,
            _ => {}
        }
        if depth == 0 {
            return self.evaluator.eval(board, is_white);
        }

        let moves = board.get_available_candidate_moves(is_white);
        let mut best_eval = if is_white { i32::MIN } else { i32::MAX };
        for mv in moves {
            if !board.make_move(is_white, mv) {
                continue;
            }
            let eval = self.get_eval(board, !is_white, depth - 1, alpha, beta);
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
