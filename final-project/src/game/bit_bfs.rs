use super::board::Board;

const TOP_MASK: u128 = 0x1FFu128 << 72;
const BOTTOM_MASK: u128 = 0x1FFu128;
const ALL_MASK: u128 = (1u128 << 81) - 1;

// Must mask these out before shifting left/right to prevent wrapping across rows.
const COL0_MASK: u128 = (1u128 << 0)
    | (1u128 << 9)
    | (1u128 << 18)
    | (1u128 << 27)
    | (1u128 << 36)
    | (1u128 << 45)
    | (1u128 << 54)
    | (1u128 << 63)
    | (1u128 << 72);
const COL8_MASK: u128 = (1u128 << 8)
    | (1u128 << 17)
    | (1u128 << 26)
    | (1u128 << 35)
    | (1u128 << 44)
    | (1u128 << 53)
    | (1u128 << 62)
    | (1u128 << 71)
    | (1u128 << 80);

impl Board {
    pub(super) fn state_valid(&self) -> bool {
        let mut white_can_reach: bool = false;

        // white pass
        let mut white_mask: u128 = 1u128 << self.white.position_idx;
        let mut prev_visited: u128 = 0;
        let mut visited: u128 = white_mask;

        while visited != prev_visited {
            if (white_mask & TOP_MASK) != 0 {
                white_can_reach = true;
                break;
            }

            let white_mask_left = ((white_mask & !COL0_MASK) >> 1) & !(self.walls.walls_right);
            let white_mask_down = (white_mask >> 9) & !(self.walls.walls_above);
            let white_mask_right =
                ((white_mask & !COL8_MASK) << 1) & !(self.walls.walls_right << 1);
            let white_mask_up = (white_mask << 9) & !(self.walls.walls_above << 9);
            white_mask =
                (white_mask_left | white_mask_down | white_mask_right | white_mask_up) & ALL_MASK;

            prev_visited = visited;
            visited |= white_mask;
        }
        if !white_can_reach {
            return false;
        }

        // black pass
        let mut black_mask: u128 = 1u128 << self.black.position_idx;
        prev_visited = 0;
        visited = black_mask;
        while visited != prev_visited {
            if (black_mask & BOTTOM_MASK) != 0 {
                return true;
            }

            let black_mask_left = ((black_mask & !COL0_MASK) >> 1) & !(self.walls.walls_right);
            let black_mask_down = (black_mask >> 9) & !(self.walls.walls_above);
            let black_mask_right =
                ((black_mask & !COL8_MASK) << 1) & !(self.walls.walls_right << 1);
            let black_mask_up = (black_mask << 9) & !(self.walls.walls_above << 9);
            black_mask =
                (black_mask_left | black_mask_down | black_mask_right | black_mask_up) & ALL_MASK;

            prev_visited = visited;
            visited |= black_mask;
        }
        false
    }

    pub(crate) fn distances(&self) -> (u8, u8) {
        let mut white_dist = 0;
        let mut black_dist = 0;

        // white pass
        let mut white_mask: u128 = 1u128 << self.white.position_idx;
        let mut prev_visited: u128 = 0;
        let mut visited: u128 = white_mask;

        while visited != prev_visited {
            if (white_mask & TOP_MASK) != 0 {
                break;
            }

            let white_mask_left = ((white_mask & !COL0_MASK) >> 1) & !(self.walls.walls_right);
            let white_mask_down = (white_mask >> 9) & !(self.walls.walls_above);
            let white_mask_right =
                ((white_mask & !COL8_MASK) << 1) & !(self.walls.walls_right << 1);
            let white_mask_up = (white_mask << 9) & !(self.walls.walls_above << 9);
            white_mask =
                (white_mask_left | white_mask_down | white_mask_right | white_mask_up) & ALL_MASK;

            prev_visited = visited;
            visited |= white_mask;
            white_dist += 1;
        }

        // black pass
        let mut black_mask: u128 = 1u128 << self.black.position_idx;
        prev_visited = 0;
        visited = black_mask;
        while visited != prev_visited {
            if (black_mask & BOTTOM_MASK) != 0 {
                break;
            }

            let black_mask_left = ((black_mask & !COL0_MASK) >> 1) & !(self.walls.walls_right);
            let black_mask_down = (black_mask >> 9) & !(self.walls.walls_above);
            let black_mask_right =
                ((black_mask & !COL8_MASK) << 1) & !(self.walls.walls_right << 1);
            let black_mask_up = (black_mask << 9) & !(self.walls.walls_above << 9);
            black_mask =
                (black_mask_left | black_mask_down | black_mask_right | black_mask_up) & ALL_MASK;

            prev_visited = visited;
            visited |= black_mask;
            black_dist += 1;
        }
        (white_dist, black_dist)
    }
}
