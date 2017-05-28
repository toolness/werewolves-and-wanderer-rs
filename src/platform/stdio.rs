use std::io::{self, Write};

use platform::AbstractPlatform;
use platform::word_wrap;
extern crate rand;

pub struct StdioPlatform;

impl AbstractPlatform for StdioPlatform {
  fn init() {
    #[cfg(target_os = "windows")]
    use platform::windows;

    #[cfg(target_os = "windows")]
    windows::enable_ansi();
  }

  fn show_prompt(prompt: &str) {
    io::stdout().write(prompt.as_bytes()).unwrap();
    io::stdout().flush().unwrap();
  }

  fn read_input() -> Option<String> {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
      Ok(_) => { Some(String::from(input.trim())) },
      Err(error) => {
        println!("Error reading input: {}", error);
        None
      },
    }
  }

  fn sleep(ms: u64) {
    let dur = ::std::time::Duration::from_millis(ms);
    ::std::thread::sleep(dur);
  }

  fn random() -> f32 {
    rand::random::<f32>()
  }

  fn is_browser() -> bool { false }

  fn clear_screen() {
    // Clear the screen.
    print!("{}[2J", 27 as char);

    // Move the cursor to the home position.
    print!("{}[H", 27 as char);
  }

  fn writeln_with_wrapping<T: AsRef<str>>(s: T) {
    word_wrap::writeln_with_wrapping(s.as_ref())
  }

  fn terminate_program() {
    ::std::process::exit(0);
  }

  fn set_main_loop_callback<F>(mut callback: F) where F: FnMut() {
    loop {
      callback();
    }
  }
}
