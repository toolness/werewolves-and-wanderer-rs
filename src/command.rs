use std::ascii::AsciiExt;
use std::io::{self, Write};

use direction::Direction;

#[derive(Debug)]
pub enum Command {
  Go(Direction),
  Look,
  Quit,
}

impl Command {
  fn from_char(c: char) -> Option<Command> {
    match c.to_ascii_lowercase() {
      'q' => { return Some(Command::Quit); },
      'n' => { return Some(Command::Go(Direction::North)); },
      's' => { return Some(Command::Go(Direction::South)); },
      'e' => { return Some(Command::Go(Direction::East)); },
      'w' => { return Some(Command::Go(Direction::West)); },
      'l' => { return Some(Command::Look); },
      _ => { return None; },
    }
  }

  pub fn get() -> Command {
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
