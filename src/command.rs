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
  pub fn get() -> Command {
    loop {
      let mut input = String::new();

      io::stdout().write(b"> ").unwrap();
      io::stdout().flush().unwrap();

      match io::stdin().read_line(&mut input) {
        Ok(_) => {
          match input.chars().next() {
            Some(k) => {
              match k.to_ascii_lowercase() {
                'q' => { return Command::Quit; },
                'n' => { return Command::Go(Direction::North); },
                's' => { return Command::Go(Direction::South); },
                'e' => { return Command::Go(Direction::East); },
                'w' => { return Command::Go(Direction::West); },
                'l' => { return Command::Look; },
                _ => {},
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
