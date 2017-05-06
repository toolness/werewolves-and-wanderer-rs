use game_state::{GameState, GameMode};
use command::{CommandProcessor, HelpInfo};
use items::Item;
use items::Item::*;
use sized_enum::SizedEnum;

use self::InventoryCommand::*;

pub struct Inventory {
  quantities: Vec<i32>,
}

impl Inventory {
  pub fn new() -> Self {
    Self { quantities: vec![0; Item::size()] }
  }

  pub fn owns(&self, item: Item) -> bool {
    self.quantities[item as usize] > 0
  }

  pub fn get_quantity(&self, item: Item) -> i32 {
    self.quantities[item as usize]
  }

  pub fn set_quantity(&mut self, item: Item, amount: i32) {
    self.quantities[item as usize] = amount;
  }

  pub fn increase(&mut self, item: Item, amount: i32) {
    self.quantities[item as usize] += amount;
  }

  pub fn decrease(&mut self, item: Item, amount: i32) {
    self.quantities[item as usize] -= amount;
  }

  pub fn obtain(&mut self, item: Item) {
    self.quantities[item as usize] = 1;
  }

  pub fn lose(&mut self, item: Item) {
    self.quantities[item as usize] = 0;
  }
}

pub enum InventoryCommand {
  Buy(Item),
  Leave,
}

impl CommandProcessor<InventoryCommand> for InventoryCommand {
  fn get_help() -> Vec<HelpInfo> {
    let buy = |item: Item| {
      format!("buy {} (${}{})",
              item, item.price(),
              if item.can_own_many() { " per unit" } else { "" })
    };

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
      '0' | 'q' => Some(Leave),
      _ => None,
    }
  }
}

impl GameState {
  fn buy_quantity(&mut self, item: Item, quantity: i32) {
    let price = item.price() * quantity;
    if self.wealth < price {
      self.accuse_player_of_cheating();
    } else {
      self.wealth -= price;
      if item.can_own_many() {
        self.items.increase(item, quantity);
        println!("You bought {} unit{} of {}.",
                 quantity, if quantity > 1 { "s" } else { "" }, item);
      } else {
        self.items.obtain(item);
        println!("You bought {}.", item);
      }
      self.print_wealth();
    }
  }

  fn buy(&mut self, item: Item) {
    if item.can_own_many() {
      self.ask_i32("How many units? ", move |state, amount| {
        if amount <= 0 {
          println!("Fine, don't buy any then.");
        } else {
          state.buy_quantity(item, amount);
        }
      });
    } else {
      self.buy_quantity(item, 1);
    }
  }

  fn process_inventory_cmd(&mut self, cmd: InventoryCommand) {
    match cmd {
      Buy(item) => {
        if !item.can_own_many() && self.items.owns(item) {
          println!("You already own {}.\n", item);
        } else {
          self.buy(item);
        }
      },
      Leave => { self.set_mode(GameMode::Primary) },
    }
  }

  pub fn tick_inventory_mode(&mut self) {
    if self.show_desc {
      println!("Provisions & inventory\n");
      self.print_wealth();
      println!("");
      InventoryCommand::show_help();
      println!("");
      self.show_desc = false;
    }

    self.ask("What do you want to buy? ", |state, input| {
      if let Some(cmd) = InventoryCommand::get_from_input(input) {
        state.process_inventory_cmd(cmd);
      }
    });
  }
}
