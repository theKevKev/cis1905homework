pub(crate) type PositionIndex = u8;

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
            walls: Walls { walls_above: 0, walls_right: 0, corners: 0 },
            white: PlayerStatus { position_idx: 4, walls_remaining: 10 },
            black: PlayerStatus { position_idx: 76, walls_remaining: 10 },
        }
    }

    pub(crate) fn walls_above_bits(&self) -> u128 { self.walls.walls_above }
    pub(crate) fn walls_right_bits(&self) -> u128 { self.walls.walls_right }
    pub(crate) fn white_pos(&self) -> PositionIndex { self.white.position_idx }
    pub(crate) fn black_pos(&self) -> PositionIndex { self.black.position_idx }
    pub(crate) fn 
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
