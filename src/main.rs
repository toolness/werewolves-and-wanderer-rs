const NUM_DIRECTIONS: usize = 4;
const NUM_ROOMS: usize = 2;

use RoomId::*;
use Direction::*;

#[derive(Debug, Copy, Clone)]
enum RoomId {
  Hallway,
  AudienceChamber,
}

trait AsId {
  fn id(self) -> usize;
}

impl AsId for RoomId {
  fn id(self) -> usize {
    self as usize
  }
}

#[derive(Debug)]
struct Map<'a, T: 'a + Copy + AsId> {
  rooms: &'a mut [Room<T>]
}

impl<'a, T: Copy + AsId> Map<'a, T> {
  pub fn new(rooms: &'a mut [Room<T>]) -> Self {
    Self { rooms: rooms }
  }

  pub fn room(&mut self, r: T) -> &mut Room<T> {
    &mut self.rooms[r.id()]
  }

  pub fn connect(&mut self, from: T, d: Direction, to: T) -> &mut Self {
    self.room(from).set_exit(d, to);
    self.room(to).set_exit(d.opposite(), from);
    self
  }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
  North,
  South,
  East,
  West,
}

impl Direction {
  pub fn opposite(self) -> Self {
    match self {
      North => South,
      South => North,
      East => West,
      West => East,
    }
  }
}

#[derive(Debug, Copy, Clone)]
struct Room<T: Copy> {
  exits: [Option<T>; NUM_DIRECTIONS],
  name: &'static str,
  description: &'static str,
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

fn main() {
  let mut rooms = [Room::new(); NUM_ROOMS];
  let mut map = Map::new(&mut rooms);

  map.room(Hallway).describe("Hallway", "");
  map.room(AudienceChamber).describe("Audience Chamber", "");

  map.connect(Hallway, South, AudienceChamber);

  println!("Map is {:?}", map);
}
