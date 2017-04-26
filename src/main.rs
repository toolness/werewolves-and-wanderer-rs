extern crate ww;

use ww::map::{Map, MapRoomId};
use ww::room::Room;
use ww::direction::Direction::*;

use RoomId::*;

const MAX_ROOMS: usize = 20;

#[derive(Debug, Copy, Clone)]
enum RoomId {
  Hallway = 1,
  AudienceChamber = 2,
}

// Ideally we'd actually just get rid of MapRoomId and add a constraint
// to Map<T> requiring that T be type-castable as usize, but I don't
// know how to do that, so...
impl MapRoomId for RoomId {
  fn room_id(self) -> usize { self as usize }
}

fn main() {
  let mut rooms = [Room::new(); MAX_ROOMS];
  let mut map = Map::new(&mut rooms);

  map.room(Hallway).describe("Hallway", "");
  map.room(AudienceChamber).describe("Audience Chamber", "");

  map.connect(Hallway, South, AudienceChamber);

  println!("Map is {:?}", map);
}
