use std::ascii::AsciiExt;

use direction::Direction;
use platform;

#[derive(Debug)]
pub enum PrimaryCommand {
  Go(Direction),
  Look,
  Quit,
}

type HelpInfo = &'static [(char, &'static str)];

pub trait CommandProcessor<T> {
  fn from_char(c: char) -> Option<T>;

  fn get_help() -> HelpInfo;

  fn prompt() -> &'static str { "> " }

  fn get() -> Option<T> {
    platform::show_prompt(Self::prompt());

    match platform::read_input() {
      Some(input) => {
        match input.chars().next() {
          Some(k) => {
            let k = k.to_ascii_lowercase();
            if k == 'h' || k == '?' {
              println!("Available commands:");

              for &(ch, desc) in Self::get_help().iter() {
                println!("{} - {}", ch, desc);
              }
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
  ('l', "look around"),
  ('q', "quit"),
];

impl CommandProcessor<PrimaryCommand> for PrimaryCommand {
  fn get_help() -> HelpInfo { PRIMARY_COMMAND_HELP }

  fn from_char(c: char) -> Option<PrimaryCommand> {
    match c {
      'n' => { return Some(PrimaryCommand::Go(Direction::North)); },
      's' => { return Some(PrimaryCommand::Go(Direction::South)); },
      'e' => { return Some(PrimaryCommand::Go(Direction::East)); },
      'w' => { return Some(PrimaryCommand::Go(Direction::West)); },
      'u' => { return Some(PrimaryCommand::Go(Direction::Up)); },
      'd' => { return Some(PrimaryCommand::Go(Direction::Down)); },
      'l' => { return Some(PrimaryCommand::Look); },
      'q' => { return Some(PrimaryCommand::Quit); },
      _ => { return None; },
    }
  }
}

