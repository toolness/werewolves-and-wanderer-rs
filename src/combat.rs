use monsters::MonsterId;
use map::RoomContents;
use game_state::{GameState, GameMode};
use command::CommandInfo;
use items::Item::*;
use direction::Direction;
use platform;

use self::FleeCommand::*;
use self::CombatPhase::*;

const CHANCE_TO_RUN: f32 = 0.3;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum CombatPhase {
  Preparation,
  Battle,
  Aftermath,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct CombatState {
  phase: CombatPhase,
  enemy: MonsterId,
  ff: i32,
}

#[derive(Debug, Copy, Clone)]
pub enum FleeCommand {
  Flee(Direction),
}

command_processor!(FleeCommand, {
  vec![
    CommandInfo::new('n', "flee north", Flee(Direction::North)),
    CommandInfo::new('s', "flee south", Flee(Direction::South)),
    CommandInfo::new('e', "flee east", Flee(Direction::East)),
    CommandInfo::new('w', "flee west", Flee(Direction::West)),
    CommandInfo::new('u', "flee up", Flee(Direction::Up)),
    CommandInfo::new('d', "flee down", Flee(Direction::Down)),
  ]
});

impl GameState {
  pub fn maybe_start_combat(&mut self) -> bool {
    if let Some(RoomContents::Terror(monster_id)) =
        self.map.room(self.curr_room).contents {
      println!("\nDanger... There is a monster here....");
      Self::pause();
      println!("\nIt is a {}!", monster_id);
      let ff = self.get_modified_ff(monster_id.ferocity_factor());
      println!("\nThe danger level is {}!!\n", ff);
      Self::pause();
      self.set_mode(GameMode::Combat(CombatState {
        phase: Preparation,
        enemy: monster_id,
        ff: ff,
      }));
      return true;
    }
    false
  }

  fn get_modified_ff(&self, base_ff: i32) -> i32 {
    let mut ff = base_ff;
    let axe = self.items.owns(Axe);
    let sword = self.items.owns(Sword);

    if self.items.owns(Armor) { ff = 3 * (ff / 4); }
    if !axe && !sword {
      ff + (ff / 5)
    } else if axe && !sword {
      4 * (ff / 5)
    } else if sword && !axe {
      3 * (ff / 4)
    } else {
      3 * (ff / 5)
    }
  }

  fn prepare(&self) {
    if self.items.owns(Armor) {
      println!("Your armor increases your chance of success.");
    }

    let axe = self.items.owns(Axe);
    let sword = self.items.owns(Sword);

    if !axe && !sword {
      println!("You have no weapons.");
      println!("You must fight with bare hands.");
    } else if axe && !sword {
      println!("You have only an axe to fight with.");
    } else if sword && !axe {
      println!("You must fight with your sword.");
    } else {
      println!("You are dual-wielding a sword and axe like a boss.");
    }
    Self::pause();
  }

  fn press_enter_to_fight(&mut self, state: CombatState) {
    self.ask("Press enter to fight! ", move |game_state, _| {
      game_state.prepare();
      println!("\n");
      game_state.curr_mode = GameMode::Combat(CombatState {
        phase: Battle,
        .. state
      });
    });
  }

  fn ask_direction_to_run(&mut self) {
    self.ask("What direction will you run? ", move |game_state, input| {
      if let Some(Flee(dir)) = FleeCommand::get_from_input(input) {
        if game_state.try_to_move(dir) {
          return;
        }
      }
      game_state.ask_again();
    });
  }

  fn tick_preparation_phase(&mut self, state: &CombatState) {
    let state = *state;
    self.ask("Will you run away like a coward? ", move |game_state, input| {
      if input.to_lowercase().starts_with('y') {
        if platform::random() <= CHANCE_TO_RUN {
          game_state.set_mode(GameMode::Primary);
          game_state.ask_direction_to_run();
          return;
        }
        println!("Your craven attempt to escape has failed.");
      }
      game_state.press_enter_to_fight(state);
    });
  }

  fn tick_battle_phase(&mut self, state: &CombatState) {
    let mut ff = state.ff;
    let mut phase = state.phase;

    if platform::random() > 0.5 {
      println!("The {} attacks!", state.enemy);
    } else {
      println!("You attack!");
    }
    Self::pause();
    if platform::random() > 0.5 {
      println!("\nYou manage to wound it!");
      ff = 5 * ff / 6;
      Self::pause();
    }
    if platform::random() > 0.5 {
      self.strength -= 5;
      if self.strength > 0 {
        println!("\nThe monster wounds you!");
      } else {
        println!("\nThe monster lands a killing blow!");
      }
      Self::pause();
    }
    if platform::random() <= 0.35 {
      phase = Aftermath;
    }
    self.curr_mode = GameMode::Combat(CombatState {
      phase: phase,
      ff: ff,
      .. *state
    });
  }

  fn tick_aftermath_phase(&mut self, state: &CombatState) {
    if platform::random_i32(0, 16) > state.ff {
      println!("\nWounded and ashamed, the {} scurries off.",
               state.enemy);
      println!("\nYou are victorious!");
      self.monsters_killed += 1;
    } else {
      println!("\nThe {} knocks you down!", state.enemy);
      println!("\nYou are at its mercy!");
      Self::pause();
      println!("\nIt appears to be in a generous mood and wanders off.");
      self.strength /= 2;
    }
    println!("\n");
    Self::pause();
    self.map.mut_room(self.curr_room).contents = None;
    self.set_mode(GameMode::Primary);
  }

  pub fn tick_combat_mode(&mut self, state: &CombatState) {
    match state.phase {
      Preparation => { self.tick_preparation_phase(state) },
      Battle => { self.tick_battle_phase(state) },
      Aftermath => { self.tick_aftermath_phase(state) },
    }
  }
}
