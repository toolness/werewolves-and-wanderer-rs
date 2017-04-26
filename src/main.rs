extern crate ww;

use ww::map::{Map, MapRoomId};
use ww::room::Room;
use ww::direction::Direction::*;
use ww::command::{PrimaryCommand, CommandProcessor};

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

fn build_world(map: &mut Map<RoomId>) {
  map.room(Hallway).describe(
    "Hallway",
    "You are in the hallway. \
     There is a door to the south. \
     Through the windows to the north you can see a secret herb garden."
  );

  map.room(AudienceChamber).describe(
    "Audience Chamber",
    "This is the audience chamber. \
     There is a window to the west. By looking to the right \
     through it you can see the entrance to the castle. \
     Doors leave this room to the north, east, and south."
  );

  map.connect(Hallway, South, AudienceChamber);
}

fn run_game(map: &mut Map<RoomId>) {
  println!("Werewolves and Wanderer\n");

  let mut curr_room = Hallway;
  let mut show_desc = true;

  loop {
    if show_desc {
      println!("{}", map.room(curr_room).description);
      show_desc = false;
    }

    match PrimaryCommand::get() {
      PrimaryCommand::Go(dir) => {
        if let Some(room) = map.room(curr_room).get_exit(dir) {
          curr_room = room;
          show_desc = true;
        } else {
          println!("You can't go that way.");
        }
      },
      PrimaryCommand::Look => { show_desc = true; }
      PrimaryCommand::Quit => { break; }
    }
  }

  println!("Farewell.");
}

fn main() {
  let mut rooms = [Room::new(); MAX_ROOMS];
  let mut map = Map::new(&mut rooms);

  build_world(&mut map);
  run_game(&mut map);
}
