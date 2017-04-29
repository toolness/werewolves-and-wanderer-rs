use util;
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
  strength: i32,
  wealth: i32,
  food: i32,
  tally: i32,
  monsters_killed: i32,
  sword: bool,
  amulet: bool,
  axe: bool,
  suit: bool,
  light: bool,
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
      strength: 100,
      wealth: 75,
      food: 0,
      tally: 0,
      monsters_killed: 0,
      sword: false,
      amulet: false,
      axe: false,
      suit: false,
      light: false,
      show_desc: true,
    }
  }

  fn print_status_report(&self) {
    println!("{}, your strength is {}.", self.player_name, self.strength);
    if self.wealth > 0 {
      println!("You have ${}.", self.wealth);
    }
    if self.food > 0 {
      println!("Your provisions sack holds {} unit{} of food.",
               self.food, if self.food == 1 { "" } else { "s" });
    }
    if self.suit {
      println!("You are wearing armor.");
    }
    let item_names = self.get_item_names();
    if item_names.len() > 0 {
      println!("You are carrying {}.",
               util::friendly_join(self.get_item_names()));
    }
  }

  fn get_item_names(&self) -> Vec<&str> {
    let mut items = Vec::new();
    if self.axe { items.push("an axe") }
    if self.sword { items.push("a sword") }
    if self.amulet { items.push("the magic amulet") }
    items
  }

  fn get_score(&self) -> i32 {
    3  * self.tally +
    5  * self.strength +
    2  * self.wealth +
    1  * self.food +
    30 * self.monsters_killed
  }

  fn finish_game(&mut self) {
    self.curr_mode = GameMode::Finished;
    println!("Your score is {}.\n", self.get_score());
    println!("Farewell.");
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
          self.finish_game();
          return;
        },
        _ => {
          platform::clear_screen();
          self.print_status_report();
          println!("");
          if self.curr_room != RoomId::Entrance && !self.light {
            println!("It is too dark to see anything.");
          } else {
            platform::writeln_with_wrapping(
              self.map.room(self.curr_room).description
            );
          }
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
          self.finish_game();
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
            platform::hide_prompt();
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
