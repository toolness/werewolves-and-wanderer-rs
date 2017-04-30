use std::fmt;
use sized_enum::SizedEnum;

use self::MonsterId::*;

const NUM_MONSTERS: usize = 4;

enum_from_primitive! {
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum MonsterId {
  Werewolf = 0,
  Fleshgorger = 1,
  Maldemer = 2,
  Dragon = 3,
}
}

impl MonsterId {
  pub fn as_str(&self) -> &'static str {
    match *self {
      Werewolf => "ferocious werewolf",
      Fleshgorger => "fanatical fleshgorger",
      Maldemer => "maloventy maldemer",
      Dragon => "devastating ice-dragon",
    }
  }

  pub fn ferocity_factor(&self) -> i32 {
    match *self {
      Werewolf => 5,
      Fleshgorger => 10,
      Maldemer => 15,
      Dragon => 20,
    }
  }
}

impl SizedEnum for MonsterId {
  fn size() -> usize { NUM_MONSTERS }
}

impl fmt::Display for MonsterId {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}
