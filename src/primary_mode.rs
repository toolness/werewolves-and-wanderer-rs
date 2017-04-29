use game_map::RoomId;
use direction::Direction;
use game_state::{GameState, GameMode};
use command::{CommandProcessor, HelpInfo};
use platform;
use util;

const PAUSE_MS: u64 = 2500;

#[derive(Debug)]
pub enum PrimaryCommand {
  Go(Direction),
  Inventory,
  Look,
  Quit,
}

static PRIMARY_COMMAND_HELP: HelpInfo = &[
  ('n', "go north"),
  ('s', "go south"),
  ('e', "go east"),
  ('w', "go west"),
  ('u', "go up"),
  ('d', "go down"),
  ('i', "inventory/buy provisions"),
  ('l', "look around"),
  ('q', "quit"),
];

impl CommandProcessor<PrimaryCommand> for PrimaryCommand {
  fn get_help() -> HelpInfo { PRIMARY_COMMAND_HELP }

  fn from_char(c: char) -> Option<PrimaryCommand> {
    match c {
      'n' => Some(PrimaryCommand::Go(Direction::North)),
      's' => Some(PrimaryCommand::Go(Direction::South)),
      'e' => Some(PrimaryCommand::Go(Direction::East)),
      'w' => Some(PrimaryCommand::Go(Direction::West)),
      'u' => Some(PrimaryCommand::Go(Direction::Up)),
      'd' => Some(PrimaryCommand::Go(Direction::Down)),
      'i' => Some(PrimaryCommand::Inventory),
      'l' => Some(PrimaryCommand::Look),
      'q' => Some(PrimaryCommand::Quit),
      _ => None,
    }
  }
}

impl<'a> GameState<'a> {
  fn print_status_report(&self) {
    println!("{}, your strength is {}.", self.player_name, self.strength);
    self.print_wealth();
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
          println!("");
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
        PrimaryCommand::Inventory => { self.set_mode(GameMode::Inventory) },
        PrimaryCommand::Look => { self.show_desc = true }
        PrimaryCommand::Quit => { self.finish_game() }
      }
    };
  }
}
