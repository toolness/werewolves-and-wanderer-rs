use direction::{Direction, NUM_DIRECTIONS};

#[derive(Debug, Copy, Clone)]
pub struct Room<T: Copy> {
  exits: [Option<T>; NUM_DIRECTIONS],
  pub name: &'static str,
  pub description: &'static str,
}

impl<T: Copy> Room<T> {
  pub fn new() -> Self {
    Self {
      exits: [None; NUM_DIRECTIONS],
      name: "",
      description: "",
    }
  }

  pub fn get_exit(self, d: Direction) -> Option<T> {
    self.exits[d as usize]
  }

  pub fn set_exit(&mut self, d: Direction, r: T) -> &mut Self {
    self.exits[d as usize] = Some(r);
    self
  }

  pub fn describe(&mut self, name: &'static str, desc: &'static str) -> &mut Self {
    self.name = name;
    self.description = desc;
    self
  }
}
