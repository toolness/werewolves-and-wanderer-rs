use game_state::{GameState, GameMode};
use command::CommandInfo;
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

#[derive(Copy, Clone)]
pub enum InventoryCommand {
  Buy(Item),
  Leave,
}

command_processor!(InventoryCommand, {
  let buy = |item: Item| {
    format!("buy {} (${}{})",
            item, item.price(),
            if item.can_own_many() { " per unit" } else { "" })
  };

  vec![
    CommandInfo::new('1', buy(Torch), Buy(Torch)),
    CommandInfo::new('2', buy(Axe), Buy(Axe)),
    CommandInfo::new('3', buy(Sword), Buy(Sword)),
    CommandInfo::new('4', buy(Food), Buy(Food)),
    CommandInfo::new('5', buy(Amulet), Buy(Amulet)),
    CommandInfo::new('6', buy(Armor), Buy(Armor)),
    CommandInfo::new('0', String::from("continue adventure"), Leave),
  ]
});

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
