extern crate ww;

use ww::map::{Map, MapRoomId};
use ww::room::Room;
use ww::direction::Direction::*;
use ww::command::{PrimaryCommand, CommandProcessor};

#[cfg(target_os = "emscripten")]
use ww::platform::emscripten;

use RoomId::*;

const MAX_ROOMS: usize = 20;

#[derive(Debug, Copy, Clone)]
enum RoomId {
  Hallway = 1,
  AudienceChamber = 2,
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

// Ideally we'd actually just get rid of MapRoomId and add a constraint
// to Map<T> requiring that T be type-castable as usize, but I don't
// know how to do that, so...
impl MapRoomId for RoomId {
  fn room_id(self) -> usize { self as usize }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum GameMode {
  Primary,
  Finished,
}

struct GameState<'a> {
  map: &'a mut Map<'a, RoomId>,
  pub curr_mode: GameMode,
  curr_room: RoomId,
  show_desc: bool,
}

impl<'a> GameState<'a> {
  pub fn new(map: &'a mut Map<'a, RoomId>) -> Self {
    Self {
      map: map,
      curr_mode: GameMode::Primary,
      curr_room: Hallway,
      show_desc: true,
    }
  }

  pub fn tick(&mut self) {
    match self.curr_mode {
      GameMode::Primary => {
        if self.show_desc {
          println!("{}", self.map.room(self.curr_room).description);
          self.show_desc = false;
        }

        if let Some(cmd) = PrimaryCommand::get() {
          match cmd {
            PrimaryCommand::Go(dir) => {
              if let Some(room) = self.map.room(self.curr_room).get_exit(dir) {
                self.curr_room = room;
                self.show_desc = true;
              } else {
                println!("You can't go that way.");
              }
            },
            PrimaryCommand::Look => { self.show_desc = true; }
            PrimaryCommand::Quit => { self.curr_mode = GameMode::Finished; }
          }
        };
      },
      GameMode::Finished => {}
    }
  }
}

fn main() {
  let mut rooms = [Room::new(); MAX_ROOMS];
  let mut map = Map::new(&mut rooms);

  build_world(&mut map);

  let mut state = GameState::new(&mut map);

  println!("Werewolves and Wanderer\n");

  #[cfg(target_os = "emscripten")]
  emscripten::set_main_loop_callback(|| state.tick());

  #[cfg(not(target_os = "emscripten"))]
  while state.curr_mode != GameMode::Finished {
    state.tick();
  }

  println!("Farewell.");
}
