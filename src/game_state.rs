use game_map::{RoomId, GameMap};
use platform;
use command::{PrimaryCommand, CommandProcessor};

const PAUSE_MS: u64 = 2500;

#[derive(Debug, PartialEq, Copy, Clone)]
enum GameMode {
  AskName,
  Primary,
  Finished,
}

pub struct GameState<'a> {
  map: &'a mut GameMap<'a>,
  curr_mode: GameMode,
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

  pub fn is_finished(&self) -> bool {
    self.curr_mode == GameMode::Finished
  }

  pub fn tick_primary_mode(&mut self) {
    if self.show_desc {
      match self.curr_room {
        RoomId::Lift => {
          println!("You have entered the lift...");
          platform::sleep(PAUSE_MS);
          println!("It slowly descends...");
          platform::sleep(PAUSE_MS);
          self.curr_room = RoomId::RearVestibule;
          return;
        },
        RoomId::Exit => {
          println!("\nYou've done it!!");
          platform::sleep(PAUSE_MS);
          println!("That was the exit from the castle.");
          platform::sleep(PAUSE_MS);
          println!("\nYou have succeeded, {}!", self.player_name);
          println!("\nYou managed to get out of the castle.");
          platform::sleep(PAUSE_MS);
          println!("\nWell done!");
          platform::sleep(PAUSE_MS);
          self.curr_mode = GameMode::Finished;
          return;
        },
        _ => {
          platform::writeln_with_wrapping(
            self.map.room(self.curr_room).description
          );
        }
      }
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
        PrimaryCommand::Quit => {
          if platform::is_browser() {
            println!("If you want to quit, close your browser.");
          } else {
            self.curr_mode = GameMode::Finished;
          }
        }
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
