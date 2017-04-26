extern crate ww;

use ww::map::{Map, MapRoomId};
use ww::room::Room;
use ww::direction::Direction::*;

use RoomId::*;

const NUM_ROOMS: usize = 2;

#[derive(Debug, Copy, Clone)]
enum RoomId {
  Hallway,
  AudienceChamber,
}

impl MapRoomId for RoomId {
  fn room_id(self) -> usize { self as usize }
}

fn main() {
  let mut rooms = [Room::new(); NUM_ROOMS];
  let mut map = Map::new(&mut rooms);

  map.room(Hallway).describe("Hallway", "");
  map.room(AudienceChamber).describe("Audience Chamber", "");

  map.connect(Hallway, South, AudienceChamber);

  println!("Map is {:?}", map);
}
