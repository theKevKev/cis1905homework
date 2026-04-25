use super::board::Board;

const TOP_MASK: u128 = 0x1FFu128 << 72;
const BOTTOM_MASK: u128 = 0x1FFu128;
const ALL_MASK: u128 = (1u128 << 81) - 1; // avoids pawns walking outside the bounds of the grid

impl Board {
    pub(super) fn state_valid(&self) -> bool {
        let mut white_can_reach: bool = false;

        // white pass
        let mut white_mask: u128 = 1u128 << self.white.position_idx;
        let mut prev_visited: u128 = 0;
        let mut visited: u128 = white_mask;

        while visited != prev_visited {
            let white_mask_left = (white_mask >> 1) & !(self.walls.walls_right);
            let white_mask_down = (white_mask >> 9) & !(self.walls.walls_above);
            let white_mask_right = (white_mask << 1) & !(self.walls.walls_right << 1);
            let white_mask_up = (white_mask << 9) & !(self.walls.walls_above << 9);
            white_mask =
                (white_mask_left | white_mask_down | white_mask_right | white_mask_up) & ALL_MASK;

            if (white_mask & TOP_MASK) != 0 {
                white_can_reach = true;
                break;
            }

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
            let black_mask_left = (black_mask >> 1) & !(self.walls.walls_right);
            let black_mask_down = (black_mask >> 9) & !(self.walls.walls_above);
            let black_mask_right = (black_mask << 1) & !(self.walls.walls_right << 1);
            let black_mask_up = (black_mask << 9) & !(self.walls.walls_above << 9);
            black_mask =
                (black_mask_left | black_mask_down | black_mask_right | black_mask_up) & ALL_MASK;

            if (black_mask & BOTTOM_MASK) != 0 {
                return true;
            }

            prev_visited = visited;
            visited |= black_mask;
        }
        false
    }
}
