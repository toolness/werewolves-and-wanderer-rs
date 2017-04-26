const NUM_DIRECTIONS: usize = 4;
const NUM_ROOMS: usize = 2;

use RoomId::*;
use Direction::*;

#[derive(Debug, Copy, Clone)]
enum RoomId {
  Hallway,
  AudienceChamber,
}

#[derive(Debug)]
struct Map {
  rooms: [Room; NUM_ROOMS]
}

impl Map {
  pub fn new() -> Self {
    Self { rooms: [Room::new(); NUM_ROOMS] }
  }

  pub fn room(&mut self, id: RoomId) -> &mut Room {
    &mut self.rooms[id as usize]
  }

  pub fn connect(&mut self, from: RoomId, d: Direction, to: RoomId) -> &mut Self {
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
struct Room {
  exits: [Option<RoomId>; NUM_DIRECTIONS],
  name: &'static str,
  description: &'static str,
}

impl Room {
  pub fn new() -> Self {
    Self {
      exits: [None; NUM_DIRECTIONS],
      name: "",
      description: "",
    }
  }

  pub fn get_exit(self, d: Direction) -> Option<RoomId> {
    self.exits[d as usize]
  }

  pub fn set_exit(&mut self, d: Direction, r: RoomId) -> &mut Self {
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
  let mut map = Map::new();

  map.room(Hallway).describe("Hallway", "");
  map.room(AudienceChamber).describe("Audience Chamber", "");

  map.connect(Hallway, South, AudienceChamber);

  println!("Map is {:?}", map);
}
