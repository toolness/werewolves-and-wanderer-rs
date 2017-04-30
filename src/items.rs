use std::fmt;

use self::Item::*;

pub enum Item {
  Torch,
  Axe,
  Sword,
  Amulet,
}

impl Item {
  pub fn price(&self) -> i32 {
    match *self {
      Torch => 15,
      Axe => 10,
      Sword => 20,
      Amulet => 30,
    }
  }

  pub fn as_str(&self) -> &'static str {
    match *self {
      Torch => "a flaming torch",
      Axe => "an axe",
      Sword => "a sword",
      Amulet => "the magic amulet",
    }
  }
}

impl fmt::Display for Item {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}
