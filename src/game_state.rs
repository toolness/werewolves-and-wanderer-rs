use game_map::{RoomId, GameMap};
use platform;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GameMode {
  AskName,
  Primary,
  Inventory,
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
      strength: 100,
      wealth: 75,
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

  pub fn tick(&mut self) {
    match self.curr_mode {
      GameMode::AskName => { self.tick_ask_name_mode() },
      GameMode::Primary => { self.tick_primary_mode() },
      GameMode::Inventory => { self.tick_inventory_mode() },
      GameMode::Finished => {}
    }
  }
}
