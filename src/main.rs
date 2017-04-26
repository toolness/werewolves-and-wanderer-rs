const NUM_DIRECTIONS: usize = 4;
const NUM_ROOMS: usize = 2;

#[derive(Debug, Copy, Clone)]
enum RoomId {
  Hallway,
  AudienceChamber,
}

#[derive(Debug)]
struct Map<'a> {
  rooms: [Room<'a>; NUM_ROOMS]
}

impl<'a> Map<'a> {
  pub fn new() -> Self {
    Self { rooms: [Room::new(); NUM_ROOMS] }
  }

  pub fn room(&mut self, id: RoomId) -> &mut Room<'a> {
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
      Direction::North => Direction::South,
      Direction::South => Direction::North,
      Direction::East => Direction::West,
      Direction::West => Direction::East,
    }
  }
}

#[derive(Debug, Copy, Clone)]
struct Room<'a> {
  exits: [Option<RoomId>; NUM_DIRECTIONS],
  name: &'a str,
  description: &'a str,
}

impl<'a> Room<'a> {
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

  pub fn describe(&mut self, name: &'a str, desc: &'a str) -> &mut Self {
    self.name = name;
    self.description = desc;
    self
  }
}

fn main() {
  let mut map = Map::new();

  map.room(RoomId::Hallway).describe("Hallway", "");
  map.room(RoomId::AudienceChamber).describe("Audience Chamber", "");

  map.connect(RoomId::Hallway, Direction::South, RoomId::AudienceChamber);

  println!("Map is {:?}", map);
}
