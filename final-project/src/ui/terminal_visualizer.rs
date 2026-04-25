use crate::game::board::Board;
use crate::game::r#move::{Move, Orientation};
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct GameState {
    pub board: Board,
    pub is_white_turn: bool,
    pub white_walls: Vec<Move>,
    pub black_walls: Vec<Move>,
}

// Expand a list of wall moves into their walls_above / walls_right bitmasks.
fn walls_to_bits(walls: &[Move]) -> (u128, u128) {
    let mut above = 0u128;
    let mut right = 0u128;
    for &mv in walls {
        if let Move::Wall { corner_idx, orientation } = mv {
            let sq = (corner_idx / 8) as u32 * 9 + (corner_idx % 8) as u32;
            match orientation {
                Orientation::Horizontal => above |= 3u128 << sq,
                Orientation::Vertical   => right |= 513u128 << sq,
            }
        }
    }
    (above, right)
}

fn bit_color(sq: u32, white_bits: u128, black_bits: u128) -> Color {
    if (white_bits >> sq) & 1 != 0 { Color::Cyan }
    else if (black_bits >> sq) & 1 != 0 { Color::Magenta }
    else { Color::Green }
}

pub fn draw_board(f: &mut Frame, area: Rect, state: &GameState) {
    let walls_above = state.board.walls_above_bits();
    let walls_right = state.board.walls_right_bits();
    let (white_above, white_right) = walls_to_bits(&state.white_walls);
    let (black_above, black_right) = walls_to_bits(&state.black_walls);

    let mut text: Vec<Line> = Vec::new();

    // visual_y=0 is top of screen = game row 8 (black's goal side)
    // visual_y=16 is bottom of screen = game row 0 (white's start)
    for visual_y in 0..17usize {
        let mut spans: Vec<Span> = Vec::new();

        for visual_x in 0..17usize {
            match (visual_x % 2, visual_y % 2) {
                // Square cell
                (0, 0) => {
                    let game_row = 8 - visual_y / 2;
                    let game_col = visual_x / 2;
                    let idx = (game_row * 9 + game_col) as u8;
                    if state.board.white_pos() == idx {
                        spans.push(Span::styled("[W]", Style::default().fg(Color::Cyan)));
                    } else if state.board.black_pos() == idx {
                        spans.push(Span::styled("[B]", Style::default().fg(Color::Magenta)));
                    } else {
                        spans.push(Span::raw("[ ]"));
                    }
                }
                // Vertical wall gap (between columns, same row)
                (1, 0) => {
                    let game_row = 8 - visual_y / 2;
                    let sq = (game_row * 9 + visual_x / 2) as u32;
                    if (walls_right >> sq) & 1 != 0 {
                        let color = bit_color(sq, white_right, black_right);
                        spans.push(Span::styled("┃", Style::default().fg(color)));
                    } else {
                        spans.push(Span::raw(" "));
                    }
                }
                // Horizontal wall gap (between rows, same column)
                (0, 1) => {
                    // lower_game_row is the row below this gap in game coords
                    let lower_game_row = 8 - (visual_y + 1) / 2;
                    let sq = (lower_game_row * 9 + visual_x / 2) as u32;
                    if (walls_above >> sq) & 1 != 0 {
                        let color = bit_color(sq, white_above, black_above);
                        spans.push(Span::styled("━━━", Style::default().fg(color)));
                    } else {
                        spans.push(Span::raw("   "));
                    }
                }
                // Corner intersection (odd x, odd y)
                _ => {
                    let lower_game_row = 8 - (visual_y + 1) / 2;
                    let sq = (lower_game_row * 9 + visual_x / 2) as u32;
                    let has_v = (walls_right >> sq) & 1 != 0;
                    let has_h = (walls_above >> sq) & 1 != 0;
                    if has_v {
                        let color = bit_color(sq, white_right, black_right);
                        spans.push(Span::styled("┃", Style::default().fg(color)));
                    } else if has_h {
                        let color = bit_color(sq, white_above, black_above);
                        spans.push(Span::styled("━", Style::default().fg(color)));
                    } else {
                        spans.push(Span::raw(" "));
                    }
                }
            }
        }
        text.push(Line::from(spans));
    }

    let turn = if state.is_white_turn { "White" } else { "Black" };
    let paragraph = Paragraph::new(text)
        .block(Block::default().title(format!(" Quoridor — {turn}'s turn ")).borders(Borders::ALL));
    f.render_widget(paragraph, area);
}
