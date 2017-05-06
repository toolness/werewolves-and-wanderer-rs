use std::ascii::AsciiExt;

macro_rules! command_processor {
  ( $command_enum:path, $block:block ) => {
    use command::CommandProcessor;

    impl CommandProcessor<$command_enum> for $command_enum {
      fn get_command_info() -> Vec<CommandInfo<$command_enum>> $block
    }
  }
}

pub struct CommandInfo<T: Copy> {
  key: char,
  desc: String,
  cmd: T,
  hidden: bool,
}

impl<T: Copy> CommandInfo<T> {
  pub fn new<S: AsRef<str>>(key: char, desc: S, cmd: T) -> Self {
    Self {
      key,
      // TODO: We might be making unnecessary copies of strings here.
      desc: String::from(desc.as_ref()),
      cmd,
      hidden: false
    }
  }

  pub fn hidden(mut self) -> Self {
    self.hidden = true;
    self
  }
}

pub trait CommandProcessor<T: Copy> {
  fn from_char(c: char) -> Option<T> {
    // A HashMap here would obviously be more efficient, but
    // since we're not going to be called very often, it's probably
    // not that big a deal.
    for info in Self::get_command_info().iter() {
      if c == info.key { return Some(info.cmd); }
    }
    None
  }

  fn get_command_info() -> Vec<CommandInfo<T>>;

  fn show_help() {
    for info in Self::get_command_info().iter().filter(|i| !i.hidden) {
      println!("  {} - {}", info.key, info.desc);
    }
  }

  fn get_from_input(input: String) -> Option<T> {
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
