mod ai;
mod game;
mod ui;

use std::io;

use game::r#move::Move;
use game::state::{check_result, GameResult, Player};
use ui::app::{setup_terminal, teardown_terminal, wait_for_quit_signal};
use ui::human::Human;
use ui::terminal_visualizer::{draw_board, GameState};

fn main() -> io::Result<()> {
    let mut terminal = setup_terminal()?;
    let mut state = GameState::new();
    let mut players: [Box<dyn Player>; 2] = [Box::new(Human), Box::new(Human)];

    'game: loop {
        terminal.draw(|f| draw_board(f, f.area(), &state))?;

        if !matches!(check_result(&state.board), GameResult::InProgress) {
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
