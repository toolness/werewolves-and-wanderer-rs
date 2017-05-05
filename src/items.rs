use std::fmt;

use self::Item::*;

#[derive(PartialEq, Copy, Clone)]
pub enum Item {
  Torch,
  Axe,
  Sword,
  Food,
  Amulet,
  Armor,
}

impl Item {
  pub fn price(&self) -> i32 {
    match *self {
      Torch => 15,
      Axe => 10,
      Sword => 20,
      Food => 4,
      Amulet => 30,
      Armor => 50,
    }
  }

  pub fn can_own_many(&self) -> bool {
    *self == Food
  }

  pub fn as_str(&self) -> &'static str {
    match *self {
      Torch => "a flaming torch",
      Axe => "an axe",
      Sword => "a sword",
      Food => "food",
      Amulet => "the magic amulet",
      Armor => "a suit of armor",
    }
  }
}

impl fmt::Display for Item {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}
