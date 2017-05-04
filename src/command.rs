use std::ascii::AsciiExt;

use platform;

pub struct HelpInfo {
  key: char,
  desc: String,
}

impl HelpInfo {
  pub fn list<T: AsRef<str>>(v: Vec<(char, T)>) -> Vec<Self> {
    v.into_iter().map(|(key, desc)| {
      // TODO: We might be making unnecessary copies of strings here.
      Self {key: key, desc: String::from(desc.as_ref())}
    }).collect()
  }
}

pub trait CommandProcessor<T> {
  fn from_char(c: char) -> Option<T>;

  fn get_help() -> Vec<HelpInfo>;

  fn show_help() {
    for info in Self::get_help().iter() {
      println!("  {} - {}", info.key, info.desc);
    }
  }

  fn get_from_input(input: String) -> Option<T> {
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
  }
}
