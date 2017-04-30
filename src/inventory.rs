use std::fmt;

use game_state::{GameState, GameMode};
use command::{CommandProcessor, HelpInfo};

use self::Item::*;

pub enum Item {
  Torch,
  Axe,
  Sword,
  Amulet,
}

impl Item {
  pub fn price(&self) -> i32 {
    match *self {
      Torch => 15,
      Axe => 10,
      Sword => 20,
      Amulet => 30,
    }
  }

  pub fn as_str(&self) -> &'static str {
    match *self {
      Torch => "a flaming torch",
      Axe => "an axe",
      Sword => "a sword",
      Amulet => "the magic amulet",
    }
  }
}

impl fmt::Display for Item {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.as_str())
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
      ('2', buy(Axe)),
      ('3', buy(Sword)),
      ('5', buy(Amulet)),
      ('0', String::from("continue adventure")),
    ])
  }

  fn from_char(c: char) -> Option<InventoryCommand> {
    match c {
      '1' => Some(InventoryCommand::Buy(Torch)),
      '2' => Some(InventoryCommand::Buy(Axe)),
      '3' => Some(InventoryCommand::Buy(Sword)),
      '5' => Some(InventoryCommand::Buy(Amulet)),
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
              Axe => self.axe = true,
              Sword => self.sword = true,
              Amulet => self.amulet = true,
            }
          }
          println!("");
        },
        InventoryCommand::Quit => { self.set_mode(GameMode::Primary) },
      }
    }
  }
}
