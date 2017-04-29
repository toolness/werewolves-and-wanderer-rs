use std::fmt;

use game_state::{GameState, GameMode};
use command::{CommandProcessor, HelpInfo};

use self::Item::*;

pub enum Item {
  Torch,
}

impl Item {
  pub fn price(&self) -> i32 {
    match *self {
      Torch => 15
    }
  }
}

impl fmt::Display for Item {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", match *self {
      Torch => "a flaming torch",
    })
  }
}

pub enum InventoryCommand {
  Buy(Item),
  Quit,
}

impl CommandProcessor<InventoryCommand> for InventoryCommand {
  fn prompt() -> &'static str { "What do you want to buy? " }

  fn get_help() -> Vec<HelpInfo> {
    let buy = |item: Item| format!("buy {} (${})", item, item.price());

    HelpInfo::list(vec![
      ('1', buy(Torch)),
      ('0', String::from("continue adventure")),
    ])
  }

  fn from_char(c: char) -> Option<InventoryCommand> {
    match c {
      '1' => Some(InventoryCommand::Buy(Torch)),
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
        InventoryCommand::Buy(item) => {
          let price = item.price();
          if price > self.wealth {
            println!("You don't have enough money to buy that.");
          } else {
            self.wealth -= price;
            println!("You bought {}.", item);
            self.print_wealth();
            match item {
              Torch => self.light = true,
            }
          }
          println!("");
        },
        InventoryCommand::Quit => { self.set_mode(GameMode::Primary) },
      }
    }
  }
}
