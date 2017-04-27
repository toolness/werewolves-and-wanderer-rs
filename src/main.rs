extern crate ww;

use ww::game_map::{self, RoomId, GameMap};
use ww::room::Room;
use ww::platform;
use ww::command::{PrimaryCommand, CommandProcessor};

#[cfg(target_os = "emscripten")]
use ww::platform::emscripten;

#[derive(Debug, PartialEq, Copy, Clone)]
enum GameMode {
  AskName,
  Primary,
  Finished,
}

struct GameState<'a> {
  map: &'a mut GameMap<'a>,
  pub curr_mode: GameMode,
  player_name: String,
  curr_room: RoomId,
  show_desc: bool,
}

impl<'a> GameState<'a> {
  pub fn new(map: &'a mut GameMap<'a>) -> Self {
    Self {
      map: map,
      player_name: String::from(""),
      curr_mode: GameMode::AskName,
      curr_room: RoomId::Entrance,
      show_desc: true,
    }
  }

  pub fn tick_primary_mode(&mut self) {
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
  }

  pub fn tick(&mut self) {
    match self.curr_mode {
      GameMode::AskName => {
        platform::show_prompt("What is your name, explorer? ");

        platform::read_input().map(|input| {
          if input.len() == 0 {
            println!("Pardon me?");
          } else {
            self.player_name = input;
            self.curr_mode = GameMode::Primary;
          }
        });
      },
      GameMode::Primary => { self.tick_primary_mode() },
      GameMode::Finished => {}
    }
  }
}

fn main() {
  let mut rooms = [Room::new(); game_map::NUM_ROOMS];
  let mut map = GameMap::new(&mut rooms);

  map.populate();

  let mut state = GameState::new(&mut map);

  println!("Werewolves and Wanderer\n");

  ::ww::platform::sleep(500);

  #[cfg(target_os = "emscripten")]
  emscripten::set_main_loop_callback(|| state.tick());

  #[cfg(not(target_os = "emscripten"))]
  while state.curr_mode != GameMode::Finished {
    state.tick();
  }

  println!("Farewell.");
}
