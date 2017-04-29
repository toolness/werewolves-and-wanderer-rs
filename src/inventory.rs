use std::fmt;

use game_state::{GameState, GameMode};
use command::{CommandProcessor, HelpInfo};

pub enum Item {
  Torch,
}

impl fmt::Display for Item {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", match *self {
      Item::Torch => "a torch",
    })
  }
}

pub enum InventoryCommand {
  Buy(Item, i32),
  Quit,
}

static INVENTORY_COMMAND_HELP: HelpInfo = &[
  ('1', "buy a flaming torch ($15)"),
  ('0', "continue adventure"),
];

impl CommandProcessor<InventoryCommand> for InventoryCommand {
  fn prompt() -> &'static str { "What do you want to buy? " }

  fn get_help() -> HelpInfo { INVENTORY_COMMAND_HELP }

  fn from_char(c: char) -> Option<InventoryCommand> {
    match c {
      '1' => Some(InventoryCommand::Buy(Item::Torch, 15)),
      _ => Some(InventoryCommand::Quit),
    }
  }
}

impl<'a> GameState<'a> {
  pub fn tick_inventory_mode(&mut self) {
    if self.show_desc {
      println!("Provisions & inventory\n");
      self.print_wealth();
      println!("");
      InventoryCommand::show_help();
      println!("");
      self.show_desc = false;
    }

    if let Some(cmd) = InventoryCommand::get() {
      match cmd {
        InventoryCommand::Buy(item, price) => {
          if price > self.wealth {
            println!("You don't have enough money to buy that.");
          } else {
            self.wealth -= price;
            println!("You bought {}.", item);
            self.print_wealth();
            match item {
              Item::Torch => self.light = true,
            }
          }
          println!("");
        },
        InventoryCommand::Quit => {
          self.show_desc = true;
          self.curr_mode = GameMode::Primary;
        }
      }
    }
  }
}
