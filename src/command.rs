use std::ascii::AsciiExt;

use direction::Direction;
use platform;

#[derive(Debug)]
pub enum PrimaryCommand {
  Go(Direction),
  Inventory,
  Look,
  Quit,
}

pub type HelpInfo = &'static [(char, &'static str)];

pub trait CommandProcessor<T> {
  fn from_char(c: char) -> Option<T>;

  fn get_help() -> HelpInfo;

  fn prompt() -> &'static str { "What do you want to do? " }

  fn show_help() {
    for &(ch, desc) in Self::get_help().iter() {
      println!("  {} - {}", ch, desc);
    }
  }

  fn get() -> Option<T> {
    platform::show_prompt(Self::prompt());

    match platform::read_input() {
      Some(input) => {
        platform::hide_prompt();
        match input.chars().next() {
          Some(k) => {
            let k = k.to_ascii_lowercase();
            if k == 'h' || k == '?' {
              println!("Here's what I understand right now:\n");
              Self::show_help();
              println!("");
              return None;
            } else if let Some(cmd) = Self::from_char(k) {
              return Some(cmd);
            }
          },
          None => {}
        }
        println!("I have no idea what you're talking about.");
        return None;
      },
      None => {
        return None;
      }
    }
  }
}

static PRIMARY_COMMAND_HELP: HelpInfo = &[
  ('n', "go north"),
  ('s', "go south"),
  ('e', "go east"),
  ('w', "go west"),
  ('u', "go up"),
  ('d', "go down"),
  ('i', "inventory/buy provisions"),
  ('l', "look around"),
  ('q', "quit"),
];

impl CommandProcessor<PrimaryCommand> for PrimaryCommand {
  fn get_help() -> HelpInfo { PRIMARY_COMMAND_HELP }

  fn from_char(c: char) -> Option<PrimaryCommand> {
    match c {
      'n' => Some(PrimaryCommand::Go(Direction::North)),
      's' => Some(PrimaryCommand::Go(Direction::South)),
      'e' => Some(PrimaryCommand::Go(Direction::East)),
      'w' => Some(PrimaryCommand::Go(Direction::West)),
      'u' => Some(PrimaryCommand::Go(Direction::Up)),
      'd' => Some(PrimaryCommand::Go(Direction::Down)),
      'i' => Some(PrimaryCommand::Inventory),
      'l' => Some(PrimaryCommand::Look),
      'q' => Some(PrimaryCommand::Quit),
      _ => None,
    }
  }
}
