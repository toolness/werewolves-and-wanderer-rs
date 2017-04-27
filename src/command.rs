use std::ascii::AsciiExt;

#[cfg(not(target_os = "emscripten"))]
use std::io::{self, Write};

use direction::Direction;

#[cfg(target_os = "emscripten")]
use emscripten::{emscripten};

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

  #[cfg(target_os = "emscripten")]
  fn show_prompt() {
    let script = format!("set_prompt({:?});", Self::prompt());
    emscripten::run_script_int(script.as_str());
  }

  #[cfg(not(target_os = "emscripten"))]
  fn show_prompt() {
    io::stdout().write(Self::prompt().as_bytes()).unwrap();
    io::stdout().flush().unwrap();
  }

  #[cfg(target_os = "emscripten")]
  fn read_input() -> Option<String> {
    if emscripten::run_script_int("has_input()") == 0 {
      None
    } else {
      Some(emscripten::run_script_string("get_input()"))
    }
  }

  #[cfg(not(target_os = "emscripten"))]
  fn read_input() -> Option<String> {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
      Ok(_) => { return Some(input); },
      Err(error) => {
        println!("Error reading input: {}", error);
        return None;
      },
    }
  }

  fn get() -> Option<T> {
    Self::show_prompt();

    match Self::read_input() {
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
      'l' => { return Some(PrimaryCommand::Look); },
      'q' => { return Some(PrimaryCommand::Quit); },
      _ => { return None; },
    }
  }
}
