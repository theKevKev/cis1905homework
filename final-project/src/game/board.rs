pub(crate) type PositionIndex = u8;

use std::sync::OnceLock;

// Zobrist random tables grouped so all are initialized from one OnceLock/PRNG sequence.
struct ZobristTables {
    white_pos: [u64; 81],
    black_pos: [u64; 81],
    white_walls_rem: [u64; 11],
    black_walls_rem: [u64; 11],
    wall_above_bit: [u64; 128],
    wall_right_bit: [u64; 128],
    corner_bit: [u64; 64],
    black_turn: u64,
}

static ZOBRIST: OnceLock<ZobristTables> = OnceLock::new();

fn get_zobrist() -> &'static ZobristTables {
    ZOBRIST.get_or_init(|| {
        let mut s: u64 = 0xDEAD_BEEF_CAFE_F00D;
        let mut next = || { s ^= s << 13; s ^= s >> 7; s ^= s << 17; s };
        ZobristTables {
            white_pos: std::array::from_fn(|_| next()),
            black_pos: std::array::from_fn(|_| next()),
            white_walls_rem: std::array::from_fn(|_| next()),
            black_walls_rem: std::array::from_fn(|_| next()),
            wall_above_bit: std::array::from_fn(|_| next()),
            wall_right_bit: std::array::from_fn(|_| next()),
            corner_bit: std::array::from_fn(|_| next()),
            black_turn: next(),
        }
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) struct Walls {
    pub(super) walls_above: u128,
    pub(super) walls_right: u128,
    pub(super) corners: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) struct PlayerStatus {
    pub(super) position_idx: PositionIndex,
    pub(super) walls_remaining: u8,
}

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub(crate) struct Board {
    pub(super) walls: Walls,
    pub(super) white: PlayerStatus,
    pub(super) black: PlayerStatus,
}

impl Board {
    pub(crate) fn new() -> Self {
        Board {
            walls: Walls {
                walls_above: 0,
                walls_right: 0,
                corners: 0,
            },
            white: PlayerStatus {
                position_idx: 4,
                walls_remaining: 10,
            },
            black: PlayerStatus {
                position_idx: 76,
                walls_remaining: 10,
            },
        }
    }

    pub(crate) fn white_pos(&self) -> PositionIndex {
        self.white.position_idx
    }
    pub(crate) fn black_pos(&self) -> PositionIndex {
        self.black.position_idx
    }
    pub(crate) fn white_walls_remaining(&self) -> u8 {
        self.white.walls_remaining
    }
    pub(crate) fn black_walls_remaining(&self) -> u8 {
        self.black.walls_remaining
    }
}

impl Board {
    pub(crate) fn zobrist_hash(&self, is_white: bool) -> u64 {
        let z = get_zobrist();
        let mut h = z.white_pos[self.white.position_idx as usize];
        h ^= z.black_pos[self.black.position_idx as usize];
        h ^= z.white_walls_rem[self.white.walls_remaining as usize];
        h ^= z.black_walls_rem[self.black.walls_remaining as usize];
        if !is_white { h ^= z.black_turn; }
        let mut wa = self.walls.walls_above;
        while wa != 0 { h ^= z.wall_above_bit[wa.trailing_zeros() as usize]; wa &= wa - 1; }
        let mut wr = self.walls.walls_right;
        while wr != 0 { h ^= z.wall_right_bit[wr.trailing_zeros() as usize]; wr &= wr - 1; }
        let mut c = self.walls.corners;
        while c != 0 { h ^= z.corner_bit[c.trailing_zeros() as usize]; c &= c - 1; }
        h
    }
}

impl Board {
    // bit-wise helpers to check for walls
    #[inline(always)]
    pub(super) fn wall_above(&self, idx: u8) -> bool {
        idx >= 72 || (self.walls.walls_above & (1u128 << idx)) != 0
    }

    #[inline(always)]
    pub(super) fn wall_below(&self, idx: u8) -> bool {
        idx < 9 || (self.walls.walls_above & (1u128 << (idx - 9))) != 0
    }

    #[inline(always)]
    pub(super) fn wall_right(&self, idx: u8) -> bool {
        (idx % 9) == 8 || (self.walls.walls_right & (1u128 << idx)) != 0
    }

    #[inline(always)]
    pub(super) fn wall_left(&self, idx: u8) -> bool {
        (idx % 9) == 0 || (self.walls.walls_right & (1u128 << (idx - 1))) != 0
    }
}
