use game_map::{RoomId, GameMap};
use combat::CombatState;
use inventory::Inventory;
use items::Item::*;
use platform;

const PAUSE_MS: u64 = 2500;
const CHEATING_FOOD_DIVISOR: i32 = 4;
const INITIAL_STRENGTH: i32 = 100;
const INITIAL_WEALTH: i32 = 75;
const STRENGTH_PER_FOOD: i32 = 10;
const TALLY_PER_MOVE: i32 = 1;
const STRENGTH_LOSS_PER_MOVE: i32 = 5;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GameMode {
  AskName,
  Primary,
  Inventory,
  EatFood,
  Combat(CombatState),
  Finished,

  #[cfg(debug_assertions)]
  Debug,
}

// We have our input callbacks have a GameState explicitly passed
// into them instead of expecting 'self' to be captured into their
// closure, because the latter leads to all kinds of lifetime
// headaches.
//
// Note also that this doesn't need to be a FnOnce, since it can
// request that it be called again, e.g. if the user input was invalid.
type InputCallback = Fn(&mut GameState, String);

pub struct GameState {
  pub map: GameMap,
  pub curr_mode: GameMode,
  pub shown_hint: bool,
  pub player_name: String,
  pub strength: i32,
  pub wealth: i32,
  pub tally: i32,
  pub monsters_killed: i32,
  pub curr_room: RoomId,
  pub show_desc: bool,
  pub items: Inventory,
  input_callback: Option<Box<InputCallback>>,
  is_processing_input: bool,
  last_input_prompt: String,
  read_input_again: bool,
}

impl GameState {
  pub fn new() -> Self {
    Self {
      map: GameMap::new(),
      player_name: String::from(""),
      curr_mode: GameMode::AskName,
      curr_room: RoomId::Entrance,
      shown_hint: false,
      strength: INITIAL_STRENGTH,
      wealth: INITIAL_WEALTH,
      tally: 0,
      monsters_killed: 0,
      show_desc: true,
      items: Inventory::new(),
      input_callback: None,
      is_processing_input: false,
      last_input_prompt: String::from(""),
      read_input_again: false,
    }
  }

  pub fn show_prompt(&mut self, prompt: &str) {
    self.last_input_prompt = String::from(prompt);
    platform::show_prompt(prompt);
  }

  pub fn ask_again(&mut self) {
    assert_eq!(self.is_processing_input, true,
               "This method must be called from an input callback");
    self.read_input_again = true;
  }

  pub fn ask_i32<F>(&mut self, question: &str, cb: F)
      where F: 'static + Fn(&mut GameState, i32) {
    self.show_prompt(question);
    self.read_input(move |state, input| {
      match input.parse::<i32>() {
        Ok(amount) => {
          cb(state, amount);
        },
        Err(_) => {
          println!("That does not even look like a number, {}.",
                   state.player_name);
          state.ask_again();
        }
      }
    });
  }

  pub fn ask<F>(&mut self, question: &str, cb: F)
      where F: 'static + Fn(&mut GameState, String) {
    self.show_prompt(question);
    self.read_input(cb);
  }

  pub fn read_input<F>(&mut self, cb: F)
      where F: 'static + Fn(&mut GameState, String) {
    assert!(self.input_callback.is_none(),
            "Program must not already be waiting for input");
    self.read_input_again = false;
    self.input_callback = Some(Box::new(cb));
  }

  pub fn can_player_see(&self) -> bool {
    self.curr_room == RoomId::Entrance || self.items.owns(Torch)
  }

  pub fn set_mode(&mut self, mode: GameMode) {
    self.show_desc = true;
    self.curr_mode = mode;
  }

  pub fn print_wealth(&self) {
    print!("You have ");
    if self.wealth > 0 {
      println!("${}.", self.wealth);
    } else {
      println!("no money.");
    }
  }

  pub fn print_food(&self) {
    let food = self.items.get_quantity(Food);
    println!("Your provisions sack holds {} unit{} of food.",
             food, if food == 1 { "" } else { "s" });
  }

  fn get_score(&self) -> i32 {
    3  * self.tally +
    5  * self.strength +
    2  * self.wealth +
    1  * self.items.get_quantity(Food) +
    30 * self.monsters_killed
  }

  pub fn finish_game(&mut self) {
    self.curr_mode = GameMode::Finished;
    println!("Your score is {}.\n", self.get_score());
    println!("Farewell.");
  }

  pub fn is_finished(&self) -> bool {
    self.curr_mode == GameMode::Finished
  }

  pub fn accuse_player_of_cheating(&mut self) {
    println!("YOU HAVE TRIED TO CHEAT ME!");
    self.wealth = 0;
    self.items.lose(Torch);
    self.items.lose(Axe);
    self.items.lose(Sword);

    let food = self.items.get_quantity(Food);
    self.items.set_quantity(Food, food / CHEATING_FOOD_DIVISOR);

    self.items.lose(Amulet);
    self.items.lose(Armor);
    Self::pause();
  }

  pub fn pause() {
    platform::sleep(PAUSE_MS);
  }

  fn die(&mut self) {
    println!("You have died.........");
    Self::pause();
    self.finish_game();
  }

  fn tick_ask_name_mode(&mut self) {
    self.ask("What is your name, explorer? ", |state, input| {
      if input.len() == 0 {
        println!("Pardon me?");
      } else {
        state.player_name = input;
        state.set_mode(GameMode::Primary);
      }
    });
  }

  fn tick_eat_food_mode(&mut self) {
    if self.show_desc {
      self.print_food();
      println!("");
      self.show_desc = false;
    }

    self.ask_i32("How many do you want to eat? ", |state, amount| {
      if amount < 0 {
        println!("GIVE ME A POSITIVE INTEGER.");
        state.ask_again();
      } else if amount == 0 {
        println!("Fine, be that way.");
        Self::pause();
        state.set_mode(GameMode::Primary);
      } else if amount > state.items.get_quantity(Food) {
        state.accuse_player_of_cheating();
        state.set_mode(GameMode::Primary);
      } else {
        println!("After some munching, you feel stronger.");
        state.items.decrease(Food, amount);
        state.strength += amount * STRENGTH_PER_FOOD;
        state.set_mode(GameMode::Primary);
        Self::pause();
      }
    });
  }

  pub fn process_move(&mut self) {
    self.tally += TALLY_PER_MOVE;
    self.strength -= STRENGTH_LOSS_PER_MOVE;
  }

  pub fn tick(&mut self) {
    let mut input_processed = false;
    let mut input_cb: Option<Box<InputCallback>> = None;

    ::std::mem::swap(&mut input_cb, &mut self.input_callback);

    if let Some(ref cb) = input_cb {
      self.is_processing_input = true;
      match platform::read_input() {
        Some(input) => {
          platform::hide_prompt();
          cb(self, input);
          // Note that at this point, self.input_callback may be
          // set again, if the callback asked for input again.
        },
        None => {
          // We're probably running in the browser and there's currently
          // no input to process.
        }
      }
      self.is_processing_input = false;
      input_processed = true;
    }

    if input_processed {
      if self.read_input_again {
        assert!(self.input_callback.is_none(),
                "Program cannot ask to re-run last input callback \
                 *and* run a new input callback simultaneously");
        self.read_input_again = false;
        self.input_callback = input_cb;
        platform::show_prompt(self.last_input_prompt.as_str());
      }

      return;
    }

    if self.strength < 1 { self.die() }

    match self.curr_mode {
      GameMode::AskName => { self.tick_ask_name_mode() },
      GameMode::Primary => { self.tick_primary_mode() },
      GameMode::Inventory => { self.tick_inventory_mode() },
      GameMode::EatFood => { self.tick_eat_food_mode() },
      GameMode::Combat(state) => { self.tick_combat_mode(&state) },
      GameMode::Finished => {},

      #[cfg(debug_assertions)]
      GameMode::Debug => { self.tick_debug_mode() },
    }
  }
}
