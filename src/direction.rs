use std::fmt;

use sized_enum::SizedEnum;

use self::Direction::*;

const NUM_DIRECTIONS: usize = 6;

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
  pub fn character(self) -> char {
    match self {
      North => 'n',
      South => 's',
      East => 'e',
      West => 'w',
      Up => 'u',
      Down => 'd',
    }
  }

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

impl fmt::Display for Direction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", match *self {
      North => "north",
      South => "south",
      East => "east",
      West => "west",
      Up => "up",
      Down => "down",
    })
  }
}
