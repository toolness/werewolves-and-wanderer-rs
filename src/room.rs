use direction::{Direction, NUM_DIRECTIONS};

#[derive(Debug, Copy, Clone)]
pub struct Room<T: Copy, C: Copy> {
  exits: [Option<T>; NUM_DIRECTIONS],
  pub name: &'static str,
  pub description: &'static str,
  pub contents: Option<C>,
}

impl<T: Copy, C: Copy> Room<T, C> {
  pub fn new() -> Self {
    Self {
      exits: [None; NUM_DIRECTIONS],
      name: "",
      description: "",
      contents: None,
    }
  }

  pub fn get_exit(self, d: Direction) -> Option<T> {
    self.exits[d as usize]
  }

  pub fn set_exit(&mut self, d: Direction, r: T) -> &mut Self {
    assert!(self.exits[d as usize].is_none());
    self.exits[d as usize] = Some(r);
    self
  }

  pub fn describe(&mut self, name: &'static str, desc: &'static str) -> &mut Self {
    assert_eq!(self.name, "");
    self.name = name;
    self.description = desc;
    self
  }
}
