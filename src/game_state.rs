use game_map::{RoomId, GameMap};
use platform;

const PAUSE_MS: u64 = 2500;
const CHEATING_FOOD_DIVISOR: i32 = 4;
const INITIAL_STRENGTH: i32 = 100;
const INITIAL_WEALTH: i32 = 75;
const STRENGTH_PER_FOOD: i32 = 5;
const TALLY_PER_MOVE: i32 = 1;
const STRENGTH_LOSS_PER_MOVE: i32 = 5;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GameMode {
  AskName,
  Primary,
  Inventory,
  EatFood,
  Finished,
}

pub struct GameState<'a> {
  pub map: &'a mut GameMap<'a>,
  pub curr_mode: GameMode,
  pub player_name: String,
  pub strength: i32,
  pub wealth: i32,
  pub food: i32,
  pub tally: i32,
  pub monsters_killed: i32,
  pub sword: bool,
  pub amulet: bool,
  pub axe: bool,
  pub suit: bool,
  pub light: bool,
  pub curr_room: RoomId,
  pub show_desc: bool,
}

impl<'a> GameState<'a> {
  pub fn new(map: &'a mut GameMap<'a>) -> Self {
    Self {
      map: map,
      player_name: String::from(""),
      curr_mode: GameMode::AskName,
      curr_room: RoomId::Entrance,
      strength: INITIAL_STRENGTH,
      wealth: INITIAL_WEALTH,
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

  pub fn can_player_see(&self) -> bool {
    self.curr_room == RoomId::Entrance || self.light
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
    println!("Your provisions sack holds {} unit{} of food.",
             self.food, if self.food == 1 { "" } else { "s" });
  }

  fn get_score(&self) -> i32 {
    3  * self.tally +
    5  * self.strength +
    2  * self.wealth +
    1  * self.food +
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
    self.light = false;
    self.axe = false;
    self.sword = false;
    self.food = self.food / CHEATING_FOOD_DIVISOR;
    self.amulet = false;
    self.suit = false;
    Self::pause();
  }

  pub fn pause() {
    platform::hide_prompt();
    platform::sleep(PAUSE_MS);
  }

  fn die(&mut self) {
    println!("You have died.........");
    Self::pause();
    self.finish_game();
  }

  fn tick_ask_name_mode(&mut self) {
    platform::show_prompt("What is your name, explorer? ");

    platform::read_input().map(|input| {
      if input.len() == 0 {
        println!("Pardon me?");
      } else {
        platform::hide_prompt();
        self.player_name = input;
        self.set_mode(GameMode::Primary);
      }
    });
  }

  fn tick_eat_food_mode(&mut self) {
    if self.show_desc {
      self.print_food();
      println!("");
      self.show_desc = false;
    }

    platform::show_prompt("How many do you want to eat? ");

    platform::read_input().map(|input| {
      match input.parse::<i32>() {
        Ok(amount) => {
          if amount < 0 {
            println!("GIVE ME A POSITIVE INTEGER.");
          } else if amount == 0 {
            println!("Fine, be that way.");
            Self::pause();
            self.set_mode(GameMode::Primary);
          } else if amount > self.food {
            self.accuse_player_of_cheating();
            self.set_mode(GameMode::Primary);
          } else {
            platform::hide_prompt();
            println!("After some munching, you feel stronger.");
            self.food -= amount;
            self.strength += amount * STRENGTH_PER_FOOD;
            self.set_mode(GameMode::Primary);
            Self::pause();
          }
        },
        Err(_) => {
          println!("That does not even look like a number, {}.",
                   self.player_name);
        }
      }
    });
  }

  pub fn process_move(&mut self) {
    self.tally += TALLY_PER_MOVE;
    self.strength -= STRENGTH_LOSS_PER_MOVE;
  }

  pub fn tick(&mut self) {
    if self.strength < 1 { self.die() }

    match self.curr_mode {
      GameMode::AskName => { self.tick_ask_name_mode() },
      GameMode::Primary => { self.tick_primary_mode() },
      GameMode::Inventory => { self.tick_inventory_mode() },
      GameMode::EatFood => { self.tick_eat_food_mode() },
      GameMode::Finished => {}
    }
  }
}
