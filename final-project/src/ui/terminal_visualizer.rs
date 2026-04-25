use crate::game::board::Board;
use crate::game::r#move::{Move, Orientation};
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

const WHITE_FG: Color = Color::Cyan;
const BLACK_FG: Color = Color::Red;
const WHITE_WALL: Color = Color::Cyan;
const BLACK_WALL: Color = Color::Red;
const LABEL_FG: Color = Color::DarkGray;

pub struct GameState {
    pub board: Board,
    pub is_white_turn: bool,
    pub white_h_walls: u128,
    pub white_v_walls: u128,
    pub black_h_walls: u128,
    pub black_v_walls: u128,
    pub white_h_corners: u64,
    pub white_v_corners: u64,
    pub black_h_corners: u64,
    pub black_v_corners: u64,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            board: Board::new(),
            is_white_turn: true,
            white_h_walls: 0,
            white_v_walls: 0,
            black_h_walls: 0,
            black_v_walls: 0,
            white_h_corners: 0,
            white_v_corners: 0,
            black_h_corners: 0,
            black_v_corners: 0,
        }
    }

    pub fn record_wall(&mut self, mv: Move, is_white: bool) {
        if let Move::Wall { corner_idx, orientation } = mv {
            let sq = (corner_idx / 8) as u32 * 9 + (corner_idx % 8) as u32;
            match orientation {
                Orientation::Horizontal => {
                    let bits = 3u128 << sq;
                    if is_white { self.white_h_walls |= bits; self.white_h_corners |= 1u64 << corner_idx; }
                    else        { self.black_h_walls |= bits; self.black_h_corners |= 1u64 << corner_idx; }
                }
                Orientation::Vertical => {
                    let bits = 513u128 << sq;
                    if is_white { self.white_v_walls |= bits; self.white_v_corners |= 1u64 << corner_idx; }
                    else        { self.black_v_walls |= bits; self.black_v_corners |= 1u64 << corner_idx; }
                }
            }
        }
    }
}

// ── per-cell span helpers ─────────────────────────────────────────────────────

fn square_cell(game_row: usize, game_col: usize, state: &GameState) -> Span<'static> {
    let idx = (game_row * 9 + game_col) as u8;
    if state.board.white_pos() == idx {
        Span::styled("[○]", Style::default().fg(WHITE_FG))
    } else if state.board.black_pos() == idx {
        Span::styled("[●]", Style::default().fg(BLACK_FG))
    } else {
        Span::raw("[ ]")
    }
}

fn vertical_wall(game_row: usize, game_col: usize, state: &GameState) -> Span<'static> {
    let sq = (game_row * 9 + game_col) as u32;
    if (state.white_v_walls >> sq) & 1 != 0 {
        Span::styled("|", Style::default().fg(WHITE_WALL))
    } else if (state.black_v_walls >> sq) & 1 != 0 {
        Span::styled("|", Style::default().fg(BLACK_WALL))
    } else {
        Span::raw(" ")
    }
}

fn horizontal_wall(lower_row: usize, game_col: usize, state: &GameState) -> Span<'static> {
    let sq = (lower_row * 9 + game_col) as u32;
    if (state.white_h_walls >> sq) & 1 != 0 {
        Span::styled("---", Style::default().fg(WHITE_WALL))
    } else if (state.black_h_walls >> sq) & 1 != 0 {
        Span::styled("---", Style::default().fg(BLACK_WALL))
    } else {
        Span::raw("   ")
    }
}

fn corner_post(lower_row: usize, game_col: usize, state: &GameState) -> Span<'static> {
    let cidx = (lower_row * 8 + game_col) as u32;
    let wh = (state.white_h_corners >> cidx) & 1 != 0;
    let bh = (state.black_h_corners >> cidx) & 1 != 0;
    let wv = (state.white_v_corners >> cidx) & 1 != 0;
    let bv = (state.black_v_corners >> cidx) & 1 != 0;
    if wh || bh {
        Span::styled("-", Style::default().fg(if wh { WHITE_WALL } else { BLACK_WALL }))
    } else if wv || bv {
        Span::styled("|", Style::default().fg(if wv { WHITE_WALL } else { BLACK_WALL }))
    } else {
        Span::styled(".", Style::default().fg(LABEL_FG))
    }
}

// ── row/section renderers ─────────────────────────────────────────────────────

fn render_header() -> Line<'static> {
    let mut spans = vec![Span::raw("   ")];
    for c in 0..9usize {
        spans.push(Span::styled(
            format!(" {} ", (b'a' + c as u8) as char),
            Style::default().fg(LABEL_FG),
        ));
        if c < 8 {
            spans.push(Span::raw(" "));
        }
    }
    Line::from(spans)
}

fn render_square_row(visual_y: usize, state: &GameState) -> Line<'static> {
    let game_row = 8 - visual_y / 2;
    let mut spans = vec![Span::styled(
        format!("{:2} ", game_row + 1),
        Style::default().fg(LABEL_FG),
    )];
    for seg in 0..17usize {
        let game_col = seg / 2;
        spans.push(if seg % 2 == 0 {
            square_cell(game_row, game_col, state)
        } else {
            vertical_wall(game_row, game_col, state)
        });
    }
    Line::from(spans)
}

fn render_gap_row(visual_y: usize, state: &GameState) -> Line<'static> {
    let lower_row = 8 - (visual_y + 1) / 2;
    let mut spans = vec![Span::raw("   ")];
    for seg in 0..17usize {
        let game_col = seg / 2;
        spans.push(if seg % 2 == 0 {
            horizontal_wall(lower_row, game_col, state)
        } else {
            corner_post(lower_row, game_col, state)
        });
    }
    Line::from(spans)
}

fn render_status(state: &GameState) -> Vec<Line<'static>> {
    let w = state.board.white_walls_remaining();
    let b = state.board.black_walls_remaining();
    vec![
        Line::default(),
        Line::from(vec![
            Span::styled("  ○ White walls: ", Style::default().fg(WHITE_FG)),
            Span::styled(w.to_string(), Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("  ● Black walls: ", Style::default().fg(BLACK_FG)),
            Span::styled(b.to_string(), Style::default().fg(Color::White)),
        ]),
    ]
}

// ── public entry point ────────────────────────────────────────────────────────

pub fn draw_board(f: &mut Frame, area: Rect, state: &GameState) {
    let mut text = vec![render_header()];

    for visual_y in 0..17usize {
        text.push(if visual_y % 2 == 0 {
            render_square_row(visual_y, state)
        } else {
            render_gap_row(visual_y, state)
        });
    }

    text.extend(render_status(state));

    let turn = if state.is_white_turn { "White ○" } else { "Black ●" };
    f.render_widget(
        Paragraph::new(text).block(
            Block::default()
                .title(format!(" Quoridor — {turn}'s turn "))
                .borders(Borders::ALL),
        ),
        area,
    );
}
