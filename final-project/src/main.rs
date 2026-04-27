mod ai;
mod game;
mod ui;

use std::io;

use ai::bot::{
    alpha_beta_bot::AlphaBetaBot,
    minimax_bot::MiniMaxBot,
    parallel_TT_bot::ParallelTTBot,
    parallelized_alpha_beta_bot::ParallelAlphaBetaBot,
    random_bot::RandomBot,
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
    if let Some(rest) = s.strip_prefix("ptt") {
        let (depth, eval) = parse_depth_eval(rest, "ptt");
        return match eval {
            "base" => Box::new(ParallelTTBot::new(depth, BaseEvaluator::new(), 22)),
            "urgent" => Box::new(ParallelTTBot::new(depth, UrgentEvaluator::new(), 22)),
            _ => Box::new(ParallelTTBot::new(depth, SimpleEvaluator::new(), 22)),
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
        "unknown player '{}' — use human, random, mm<n>[eval], ab<n>[eval], p<n>[eval], ptt<n>[eval]",
        s
    );
}

#[cfg(test)]
mod bench {
    use crate::ai::bot::minimax_bot::MiniMaxBot;
    use crate::ai::bot::parallel_TT_bot::ParallelTTBot;
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

    fn fmt_ms(d: Option<Duration>, width: usize) -> String {
        match d {
            Some(d) => format!("{:>width$.2}", ms(d)),
            None => format!("{:>width$}", "(skipped)"),
        }
    }

    fn fmt_speedup(num: Option<Duration>, den: Option<Duration>, width: usize) -> String {
        match (num, den) {
            (Some(n), Some(d)) => format!("{:>width$.1}x", ms(n) / ms(d)),
            _ => format!("{:>width$}", "-"),
        }
    }

    fn run_table(label: &str, board: &Board) {
        println!("\n-- {} (black to move) --", label);
        println!(
            "{:<7} {:>13} {:>15} {:>12} {:>12} {:>9} {:>9} {:>9}",
            "depth", "minimax (ms)", "alpha-beta (ms)", "par-ab (ms)", "par-tt (ms)",
            "mm/ab", "ab/par", "par/tt"
        );
        println!("{}", "-".repeat(92));
        for depth in 1u8..=7 {
            let mm = if depth <= 4 {
                Some(time_bot(&mut MiniMaxBot::new(depth, SimpleEvaluator::new()), board, false))
            } else {
                None
            };
            let ab = if depth <= 5 {
                Some(time_bot(&mut AlphaBetaBot::new(depth, SimpleEvaluator::new()), board, false))
            } else {
                None
            };
            let par =
                time_bot(&mut ParallelAlphaBetaBot::new(depth, SimpleEvaluator::new()), board, false);
            let ptt = time_bot(
                &mut ParallelTTBot::new(depth, SimpleEvaluator::new(), 22),
                board,
                false,
            );
            println!(
                "{:<7} {} {} {} {} {} {} {}",
                depth,
                fmt_ms(mm, 13),
                fmt_ms(ab, 15),
                fmt_ms(Some(par), 12),
                fmt_ms(Some(ptt), 12),
                fmt_speedup(mm, ab, 9),
                fmt_speedup(ab, Some(par), 9),
                fmt_speedup(Some(par), Some(ptt), 9),
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
