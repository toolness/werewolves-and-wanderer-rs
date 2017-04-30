use game_state::{GameState, GameMode};
use command::{CommandProcessor, HelpInfo};
use items::Item;
use items::Item::*;

use self::InventoryCommand::*;

pub enum InventoryCommand {
  Buy(Item),
  Leave,
}

impl CommandProcessor<InventoryCommand> for InventoryCommand {
  fn prompt() -> &'static str { "What do you want to buy? " }

  fn get_help() -> Vec<HelpInfo> {
    let buy = |item: Item| format!("buy {} (${})", item, item.price());

    HelpInfo::list(vec![
      ('1', buy(Torch)),
      ('2', buy(Axe)),
      ('3', buy(Sword)),
      ('4', buy(Food)),
      ('5', buy(Amulet)),
      ('6', buy(Armor)),
      ('0', String::from("continue adventure")),
    ])
  }

  fn from_char(c: char) -> Option<InventoryCommand> {
    match c {
      '1' => Some(Buy(Torch)),
      '2' => Some(Buy(Axe)),
      '3' => Some(Buy(Sword)),
      '4' => Some(Buy(Food)),
      '5' => Some(Buy(Amulet)),
      '6' => Some(Buy(Armor)),
      _ => Some(Leave),
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
        Buy(item) => {
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
              Food => self.food += 1,
              Amulet => self.amulet = true,
              Armor => self.suit = true,
            }
          }
          println!("");
        },
        Leave => { self.set_mode(GameMode::Primary) },
      }
    }
  }
}
