use map::{RoomId, RoomContents};
use sized_enum::SizedEnum;
use direction::Direction;
use game_state::{GameState, GameMode};
use command::CommandInfo;
use items::Item::*;
use platform;
use util;

use self::PrimaryCommand::*;

const MIN_STRENGTH_WARNING: i32 = 10;

#[derive(Debug, Copy, Clone)]
pub enum PrimaryCommand {
  Go(Direction),
  Inventory,
  Look,
  EatFood,
  MagicAmulet,
  PickUpTreasure,
  Quit,

  #[cfg(debug_assertions)]
  Debug,
}

command_processor!(PrimaryCommand, {
  [
    Direction::iter().map(|dir| {
      CommandInfo::new(dir.character(), format!("go {}", dir), Go(dir))
    }).collect(),
    vec![
      CommandInfo::new('c', "consume food", EatFood),
      CommandInfo::new('m', "use magic amulet (if equipped)", MagicAmulet),
      CommandInfo::new('i', "inventory/buy provisions", Inventory),
      CommandInfo::new('p', "pick up treasure", PickUpTreasure),
      CommandInfo::new('l', "look around", Look),
      CommandInfo::new('q', "quit", Quit),

      #[cfg(debug_assertions)]
      CommandInfo::new('`', "debug mode", Debug).hidden(),
    ],
  ].concat()
});

impl GameState {
  fn print_status_report(&self) {
    if self.strength <= MIN_STRENGTH_WARNING {
      println!("Warning, {}! Your strength is running low.\n",
               self.player_name);
    }
    println!("{}, your strength is {}.", self.player_name, self.strength);
    self.print_wealth();
    if self.items.get_quantity(Food) > 0 { self.print_food(); }
    if self.items.owns(Armor) {
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
    for &item in [Axe, Sword, Amulet].iter() {
      if self.items.owns(item) { items.push(item.as_str()) }
    }
    items
  }

  fn use_amulet(&mut self) {
    if self.items.owns(Amulet) {
      platform::writeln_with_wrapping(
        "You invoke the magic amulet and are whisked \
         away to somewhere else..."
      );
      Self::pause();
      loop {
        let room_id = RoomId::random();
        if room_id != self.curr_room {
          self.curr_room = room_id;
          self.show_desc = true;
          self.process_move();
          break;
        }
      }
    } else {
      println!("You don't have the amulet, {}.", self.player_name);
    }
  }

  fn describe_room(&self) {
    let room = self.map.room(self.curr_room);
    platform::writeln_with_wrapping(room.description);
    if let Some(RoomContents::Treasure(amount)) = room.contents {
      println!("\nThere is treasure here worth ${}.", amount);
    }
  }

  fn process_cmd(&mut self, cmd: PrimaryCommand) {
    match cmd {
      Go(dir) => { self.try_to_move(dir); },
      Inventory => { self.set_mode(GameMode::Inventory) },
      PickUpTreasure => {
        if !self.can_player_see() {
          println!("It's too dark to see any treasure here.");
        } else if let Some(RoomContents::Treasure(amt)) =
            self.map.room(self.curr_room).contents {
          println!("You are now ${} richer.", amt);
          self.wealth += amt as i32;
          self.map.mut_room(self.curr_room).contents = None;
          self.process_move();
        } else {
          println!("There is no treasure to pick up here.");
        }
      },
      Look => { self.show_desc = true },
      EatFood => {
        if !self.items.owns(Food) {
          println!("You have no food!");
        } else {
          self.set_mode(GameMode::EatFood);
        }
      },
      MagicAmulet => { self.use_amulet() },
      Quit => { self.finish_game() },

      #[cfg(debug_assertions)]
      Debug => { self.set_mode(GameMode::Debug) },
    }
  }

  pub fn tick_primary_mode(&mut self) {
    if self.show_desc {
      match self.curr_room {
        RoomId::Lift => {
          println!("You have entered the lift...");
          Self::pause();
          println!("It slowly descends...");
          Self::pause();
          self.curr_room = RoomId::RearVestibule;
          return;
        },
        RoomId::Exit => {
          println!("\nYou've done it!!");
          Self::pause();
          println!("That was the exit from the castle.");
          Self::pause();
          println!("\nYou have succeeded, {}!", self.player_name);
          println!("\nYou managed to get out of the castle.");
          Self::pause();
          println!("\nWell done!");
          Self::pause();
          self.finish_game();
          return;
        },
        _ => {
          platform::clear_screen();
          self.print_status_report();
          println!("");
          if !self.can_player_see() {
            println!("It is too dark to see anything.");
          } else {
            self.describe_room();
            if self.maybe_start_combat() {
              return;
            }
          }
          if !self.shown_hint {
            println!("\n(You can press 'h' for help at any time.)");
            self.shown_hint = true;
          }
          println!("");
        }
      }
      self.show_desc = false;
    }

    self.ask("What do you want to do? ", |state, input| {
      if let Some(cmd) = PrimaryCommand::get_from_input(input) {
        state.process_cmd(cmd);
      };
    });
  }
}
