use std::ascii::AsciiExt;

use platform;

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
