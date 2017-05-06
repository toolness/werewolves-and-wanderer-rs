const NUM_DIRECTIONS: usize = 6;

use sized_enum::SizedEnum;

use self::Direction::*;

enum_from_primitive! {
#[derive(Debug, Copy, Clone)]
pub enum Direction {
  North,
  South,
  East,
  West,
  Up,
  Down,
}
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

impl SizedEnum for Direction {
  fn size() -> usize { NUM_DIRECTIONS }
}
