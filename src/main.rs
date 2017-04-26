const NUM_DIRECTIONS: usize = 4;

enum Direction {
  North,
  South,
  East,
  West,
}

struct Room<'a> {
  exits: [Option<&'a Room<'a>>; NUM_DIRECTIONS],
  name: &'a str,
  description: &'a str,
}

impl<'a> Room<'a> {
  pub fn new() -> Self {
    Self {
      exits: [None, None, None, None],
      name: "",
      description: "",
    }
  }

  pub fn get_exit(self, d: Direction) -> Option<&'a Room<'a>> {
    self.exits[d as usize]
  }

  pub fn set_exit(&mut self, d: Direction, room: &'a Room<'a>) -> &mut Self {
    self.exits[d as usize] = Some(room);
    self
  }
}

fn main() {
  let mut b = Room::new();
  let mut a = Room::new();

  a.set_exit(Direction::North, &mut b);
}
