use monsters::MonsterId;
use map::RoomContents;
use game_state::{GameState, GameMode};
use command::CommandInfo;
use items::Item::*;
use direction::Direction;
use sized_enum::SizedEnum;
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
  Direction::iter().map(|dir| {
    CommandInfo::new(dir.character(), format!("flee {}", dir), Flee(dir))
  }).collect()
});

impl GameState {
  pub fn maybe_start_combat(&mut self) -> bool {
    if let Some(RoomContents::Terror(monster_id)) =
        self.map.room(self.curr_room).contents {
      wrapln!("\nDanger... There is a monster here....");
      Self::pause();
      wrapln!("\nIt is a {}!", monster_id);
      let ff = self.get_modified_ff(monster_id.ferocity_factor());
      wrapln!("\nThe danger level is {}!!\n", ff);
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
      wrapln!("Your armor increases your chance of success.");
    }

    let axe = self.items.owns(Axe);
    let sword = self.items.owns(Sword);

    if !axe && !sword {
      wrapln!("You have no weapons.");
      wrapln!("You must fight with bare hands.");
    } else if axe && !sword {
      wrapln!("You have only an axe to fight with.");
    } else if sword && !axe {
      wrapln!("You must fight with your sword.");
    } else {
      wrapln!("You are dual-wielding a sword and axe like a boss.");
    }
    Self::pause();
  }

  fn press_enter_to_fight(&mut self, state: CombatState) {
    self.ask("Press enter to fight! ", move |game_state, _| {
      game_state.prepare();
      wrapln!("\n");
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
      let lower_input = input.to_lowercase();
      if lower_input.starts_with('y') {
        if platform::random() <= CHANCE_TO_RUN {
          game_state.set_mode(GameMode::Primary);
          game_state.ask_direction_to_run();
          return;
        }
        wrapln!("Your craven attempt to escape has failed.");
      } else if lower_input.starts_with('n') {
        wrapln!("Awesome.");
      } else {
        wrapln!("Please answer 'yes' or 'no'.");
        game_state.ask_again();
        return;
      }
      game_state.press_enter_to_fight(state);
    });
  }

  fn tick_battle_phase(&mut self, state: &CombatState) {
    let mut ff = state.ff;
    let mut phase = state.phase;

    if platform::random() > 0.5 {
      wrapln!("The {} attacks!", state.enemy);
    } else {
      wrapln!("You attack!");
    }
    Self::pause();
    if platform::random() > 0.5 {
      wrapln!("\nYou manage to wound it!");
      ff = 5 * ff / 6;
      Self::pause();
    }
    if platform::random() > 0.5 {
      self.strength -= 5;
      if self.strength > 0 {
        wrapln!("\nThe monster wounds you!");
      } else {
        wrapln!("\nThe monster lands a killing blow!");
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
      wrapln!("\nWounded and ashamed, the {} scurries off.",
              state.enemy);
      wrapln!("\nYou are victorious!");
      self.monsters_killed += 1;
    } else {
      wrapln!("\nThe {} knocks you down!", state.enemy);
      wrapln!("\nYou are at its mercy!");
      Self::pause();
      wrapln!("\nIt appears to be in a generous mood and wanders off.");
      self.strength /= 2;
    }
    wrapln!("\n");
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
