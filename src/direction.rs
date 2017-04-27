pub const NUM_DIRECTIONS: usize = 6;

use self::Direction::*;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
  North,
  South,
  East,
  West,
  Up,
  Down,
}

impl Direction {
  pub fn opposite(self) -> Self {
    match self {
      North => South,
      South => North,
      East => West,
      West => East,
      Up => Down,
      Down => Up,
    }
  }
}
