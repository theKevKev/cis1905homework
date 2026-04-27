use std::io::{self, Write};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{Clear, ClearType, LeaveAlternateScreen, disable_raw_mode},
};

use crate::game::board::Board;
use crate::game::r#move::{Move, Orientation};
use crate::game::player::Player;

pub struct Human;

impl Player for Human {
    fn get_move(&mut self, board: &Board, is_white: bool) -> Move {
        let label = if is_white { "White" } else { "Black" };
        let mut input = String::new();
        let mut error: Option<String> = None;

        loop {
            let (_, rows) = crossterm::terminal::size().unwrap_or((80, 30));
            let prompt_row = rows.saturating_sub(3);

            execute!(
                io::stdout(),
                cursor::MoveTo(0, prompt_row),
                Clear(ClearType::FromCursorDown),
                cursor::Show,
            )
            .unwrap();

            if let Some(ref e) = error {
                execute!(io::stdout(), cursor::MoveTo(0, prompt_row)).unwrap();
                print!("  ! {e}");
                execute!(io::stdout(), cursor::MoveTo(0, prompt_row + 1)).unwrap();
            } else {
                execute!(io::stdout(), cursor::MoveTo(0, prompt_row + 1)).unwrap();
            }

            let display_input = &input;
            print!("  [{label}] move (e5 / e5h / e5v): {display_input}");
            io::stdout().flush().unwrap();

            // Read a single key in raw mode.
            if let Ok(Event::Key(key)) = event::read() {
                // Ctrl-C / Ctrl-D / Escape → clean exit
                let is_ctrl_exit = key.modifiers.contains(KeyModifiers::CONTROL)
                    && (key.code == KeyCode::Char('c') || key.code == KeyCode::Char('d'));
                if is_ctrl_exit || key.code == KeyCode::Esc {
                    let _ = disable_raw_mode();
                    let _ = execute!(io::stdout(), LeaveAlternateScreen, cursor::Show);
                    std::process::exit(0);
                }

                match key.code {
                    KeyCode::Enter => {
                        let line = input.trim().to_string();
                        input.clear();

                        match parse_move(&line, board, is_white) {
                            None => {
                                error = Some(format!(
                                    "'{}' is not valid — use e5 to move pawn, e5h or e5v to place a wall.",
                                    line
                                ));
                            }
                            Some(mv) => {
                                let candidates = board.get_available_candidate_moves(is_white);
                                if !candidates.contains(&mv) {
                                    error = Some("Illegal move.".to_string());
                                    continue;
                                }
                                if let Move::Wall { .. } = mv {
                                    let mut test = *board;
                                    if !test.make_move(is_white, mv) {
                                        error = Some(
                                            "Illegal wall — it would trap a player.".to_string(),
                                        );
                                        continue;
                                    }
                                }
                                execute!(io::stdout(), cursor::Hide).unwrap();
                                return mv;
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        input.pop();
                        error = None;
                    }
                    KeyCode::Char(c) => {
                        input.push(c);
                        error = None;
                    }
                    _ => {}
                }
            }
        }
    }
}

fn parse_move(s: &str, board: &Board, is_white: bool) -> Option<Move> {
    let s = s.to_ascii_lowercase();
    let b = s.as_bytes();
    if b.len() < 2 {
        return None;
    }

    let col = b[0].wrapping_sub(b'a');
    let row = b[1].wrapping_sub(b'1');
    if col >= 9 || row >= 9 {
        return None;
    }

    match b.len() {
        2 => {
            let to = row * 9 + col;
            let from = if is_white {
                board.white_pos()
            } else {
                board.black_pos()
            };
            Some(Move::Pawn { from, to })
        }
        3 => {
            if col >= 8 || row >= 8 {
                return None;
            }
            let corner_idx = row * 8 + col;
            match b[2] {
                b'h' => Some(Move::Wall {
                    corner_idx,
                    orientation: Orientation::Horizontal,
                }),
                b'v' => Some(Move::Wall {
                    corner_idx,
                    orientation: Orientation::Vertical,
                }),
                _ => None,
            }
        }
        _ => None,
    }
}
