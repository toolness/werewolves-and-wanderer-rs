use std::ascii::AsciiExt;
use std::io::{self, Write};

use direction::Direction;

#[derive(Debug)]
pub enum PrimaryCommand {
  Go(Direction),
  Look,
  Quit,
}

pub trait CommandProcessor<T> {
  fn from_char(c: char) -> Option<T>;

  fn get() -> T {
    loop {
      let mut input = String::new();

      io::stdout().write(b"> ").unwrap();
      io::stdout().flush().unwrap();

      match io::stdin().read_line(&mut input) {
        Ok(_) => {
          match input.chars().next() {
            Some(k) => {
              if let Some(cmd) = Self::from_char(k) {
                return cmd;
              }
            },
            None => {}
          }
          println!("I have no idea what you're talking about.");
        },
        Err(error) => {
          println!("Error reading input: {}", error);
        }
      }
    }
  }
}

impl CommandProcessor<PrimaryCommand> for PrimaryCommand {
  fn from_char(c: char) -> Option<PrimaryCommand> {
    match c.to_ascii_lowercase() {
      'q' => { return Some(PrimaryCommand::Quit); },
      'n' => { return Some(PrimaryCommand::Go(Direction::North)); },
      's' => { return Some(PrimaryCommand::Go(Direction::South)); },
      'e' => { return Some(PrimaryCommand::Go(Direction::East)); },
      'w' => { return Some(PrimaryCommand::Go(Direction::West)); },
      'l' => { return Some(PrimaryCommand::Look); },
      _ => { return None; },
    }
  }
}
