mod ai;
mod game;
mod ui;

use std::io;

use ai::bot::{
    alpha_beta_bot::AlphaBetaBot, minimax_bot::MiniMaxBot, pab_extend_bot::PABExtendBot,
    parallelized_alpha_beta_bot::ParallelAlphaBetaBot, random_bot::RandomBot,
};
use ai::evalulator::{BaseEvaluator, SimpleEvaluator, UrgentEvaluator};

use game::r#move::Move;
use game::player::Player;
use game::state::GameResult;
use ui::app::{setup_terminal, teardown_terminal, wait_for_quit_signal};
use ui::human::Human;
use ui::terminal_visualizer::{GameState, draw_board, draw_success_screen};

fn main() -> io::Result<()> {
    let mut terminal = setup_terminal()?;
    let mut state = GameState::new();

    let args: Vec<String> = std::env::args().collect();
    let mut players: [Box<dyn Player>; 2] = [
        make_player(args.get(1).map(String::as_str).unwrap_or("human")),
        make_player(args.get(2).map(String::as_str).unwrap_or("ab5")),
    ];

    'game: loop {
        terminal.draw(|f| draw_board(f, f.area(), &state))?;

        if !matches!(&state.board.check_result(), GameResult::InProgress) {
            terminal.draw(|f| draw_success_screen(f, f.area(), &state))?;
            wait_for_quit_signal()?;
            break 'game;
        }

        let turn = if state.is_white_turn { 0 } else { 1 };
        let mv = players[turn].get_move(&state.board, state.is_white_turn);

        state.board.make_move(state.is_white_turn, mv);
        if matches!(mv, Move::Wall { .. }) {
            state.record_wall(mv, state.is_white_turn);
        }
        state.is_white_turn = !state.is_white_turn;
    }

    teardown_terminal(&mut terminal)
}

// AI-written code for input parsing and benchmarking: thanks to claude
// Usage: cargo run [--release] -- [white] [black]
//   human, random, mm<depth>[eval], ab<depth>[eval]
//   eval: base, simple (default), urgent
//   e.g. cargo run --release -- human ab7
//        cargo run -- mm3base ab5urgent
fn parse_depth_eval<'a>(s: &'a str, prefix: &str) -> (u8, &'a str) {
    let split = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
    let depth: u8 = s[..split].parse().unwrap_or_else(|_| {
        panic!(
            "{} depth must be a number, e.g. {}3 or {}3urgent",
            prefix, prefix, prefix
        )
    });
    (depth, &s[split..])
}

fn make_player(s: &str) -> Box<dyn Player> {
    if s == "human" {
        return Box::new(Human);
    }
    if s == "random" {
        return Box::new(RandomBot::new());
    }
    if let Some(rest) = s.strip_prefix("mm") {
        let (depth, eval) = parse_depth_eval(rest, "mm");
        return match eval {
            "base" => Box::new(MiniMaxBot::new(depth, BaseEvaluator::new())),
            "urgent" => Box::new(MiniMaxBot::new(depth, UrgentEvaluator::new())),
            _ => Box::new(MiniMaxBot::new(depth, SimpleEvaluator::new())),
        };
    }
    if let Some(rest) = s.strip_prefix("ab") {
        let (depth, eval) = parse_depth_eval(rest, "ab");
        return match eval {
            "base" => Box::new(AlphaBetaBot::new(depth, BaseEvaluator::new())),
            "urgent" => Box::new(AlphaBetaBot::new(depth, UrgentEvaluator::new())),
            _ => Box::new(AlphaBetaBot::new(depth, SimpleEvaluator::new())),
        };
    }
    if let Some(rest) = s.strip_prefix("e") {
        let (depth, eval) = parse_depth_eval(rest, "e");
        return match eval {
            "base" => Box::new(PABExtendBot::new(depth, BaseEvaluator::new())),
            "urgent" => Box::new(PABExtendBot::new(depth, UrgentEvaluator::new())),
            _ => Box::new(PABExtendBot::new(depth, SimpleEvaluator::new())),
        };
    }
    if let Some(rest) = s.strip_prefix("p") {
        let (depth, eval) = parse_depth_eval(rest, "p");
        return match eval {
            "base" => Box::new(ParallelAlphaBetaBot::new(depth, BaseEvaluator::new())),
            "urgent" => Box::new(ParallelAlphaBetaBot::new(depth, UrgentEvaluator::new())),
            _ => Box::new(ParallelAlphaBetaBot::new(depth, SimpleEvaluator::new())),
        };
    }
    panic!(
        "unknown player '{}' — use human, random, mm<n>[base|simple|urgent], or ab<n>[base|simple|urgent]",
        s
    );
}

#[cfg(test)]
mod bench {
    use crate::ai::bot::minimax_bot::MiniMaxBot;
    use crate::ai::bot::parallelized_alpha_beta_bot::ParallelAlphaBetaBot;
    use crate::ai::{bot::alpha_beta_bot::AlphaBetaBot, evalulator::SimpleEvaluator};
    use crate::game::board::Board;
    use crate::game::r#move::Move;
    use crate::game::player::Player;
    use std::time::{Duration, Instant};

    fn time_bot(bot: &mut dyn Player, board: &Board, is_white: bool) -> Duration {
        let start = Instant::now();
        bot.get_move(board, is_white);
        start.elapsed()
    }

    fn ms(d: Duration) -> f64 {
        d.as_secs_f64() * 1000.0
    }

    fn run_table(label: &str, board: &Board) {
        println!("\n-- {} (black to move) --", label);
        println!(
            "{:<7} {:>13} {:>15} {:>14} {:>10} {:>10}",
            "depth", "minimax (ms)", "alpha-beta (ms)", "par-ab (ms)", "mm/ab", "ab/par"
        );
        println!("{}", "-".repeat(75));
        for depth in 1u8..=7 {
            let mm = if depth <= 4 {
                Some(time_bot(
                    &mut MiniMaxBot::new(depth, SimpleEvaluator::new()),
                    board,
                    false,
                ))
            } else {
                None
            };
            let ab = if depth <= 5 {
                Some(time_bot(
                    &mut AlphaBetaBot::new(depth, SimpleEvaluator::new()),
                    board,
                    false,
                ))
            } else {
                None
            };
            let par = time_bot(
                &mut ParallelAlphaBetaBot::new(depth, SimpleEvaluator::new()),
                board,
                false,
            );
            let par_ms = ms(par);
            let ab_str = ab
                .map(|d| format!("{:>15.2}", ms(d)))
                .unwrap_or_else(|| format!("{:>15}", "(skipped)"));
            let mm_str = mm
                .map(|d| format!("{:>13.2}", ms(d)))
                .unwrap_or_else(|| format!("{:>13}", "(skipped)"));
            let mm_ab_str = match (mm, ab) {
                (Some(m), Some(a)) => format!("{:>9.1}x", ms(m) / ms(a)),
                _ => format!("{:>10}", "-"),
            };
            let ab_par_str = match ab {
                Some(a) => format!("{:>9.1}x", ms(a) / par_ms),
                None => format!("{:>10}", "-"),
            };
            println!(
                "{:<7} {} {} {:>14.2} {} {}",
                depth, mm_str, ab_str, par_ms, mm_ab_str, ab_par_str
            );
        }
    }

    #[test]
    fn compare() {
        // Position 1: white just played e2, black to respond
        let mut board1 = Board::new();
        board1.make_move(true, Move::Pawn { from: 4, to: 13 });
        run_table("after white e2", &board1);

        // Position 2: white e2, black e8, white e3 — black to respond
        let mut board2 = board1;
        board2.make_move(false, Move::Pawn { from: 76, to: 67 });
        board2.make_move(true, Move::Pawn { from: 13, to: 22 });
        run_table("after white e3", &board2);

        // Position 3: white e2, black e8, white e3, black e7, white e4 — black to respond
        let mut board3 = board2;
        board3.make_move(false, Move::Pawn { from: 67, to: 58 });
        board3.make_move(true, Move::Pawn { from: 22, to: 31 });
        run_table("after white e4", &board3);
    }
}
