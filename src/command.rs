use std::vec::Vec;
use std::ascii::AsciiExt;
use std::io::{self, Write};

use direction::Direction;

#[derive(Debug)]
pub enum PrimaryCommand {
  Go(Direction),
  Look,
  Quit,
}

type HelpInfo = Vec<(char, &'static str)>;

pub trait CommandProcessor<T> {
  fn from_char(c: char) -> Option<T>;

  fn get_help() -> HelpInfo;

  fn get() -> T {
    loop {
      let mut input = String::new();

      io::stdout().write(b"> ").unwrap();
      io::stdout().flush().unwrap();

      match io::stdin().read_line(&mut input) {
        Ok(_) => {
          match input.chars().next() {
            Some(k) => {
              let k = k.to_ascii_lowercase();
              if k == 'h' || k == '?' {
                println!("Available commands:");

                for &(ch, desc) in Self::get_help().iter() {
                  println!("{} - {}", ch, desc);
                }
                continue;
              } else if let Some(cmd) = Self::from_char(k) {
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
  fn get_help() -> HelpInfo {
    vec![
      ('n', "go north"),
      ('s', "go south"),
      ('e', "go east"),
      ('w', "go west"),
      ('q', "quit"),
    ]
  }

  fn from_char(c: char) -> Option<PrimaryCommand> {
    match c {
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
