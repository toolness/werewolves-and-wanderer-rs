#[cfg(target_os = "emscripten")]
pub mod emscripten;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(not(target_os = "emscripten"))]
pub mod word_wrap;

#[cfg(not(target_os = "emscripten"))]
use std::io::{self, Write};

#[cfg(not(target_os = "emscripten"))]
extern crate rand;

#[cfg(target_os = "emscripten")]
pub fn show_prompt(prompt: &str) {
  let script = format!("set_prompt({:?});", prompt);
  emscripten::run_script(script.as_str());
}

#[cfg(not(target_os = "emscripten"))]
pub fn show_prompt(prompt: &str) {
  io::stdout().write(prompt.as_bytes()).unwrap();
  io::stdout().flush().unwrap();
}

#[cfg(target_os = "emscripten")]
pub fn read_input() -> Option<String> {
  if emscripten::run_script_int("has_input()") == 0 {
    None
  } else {
    Some(emscripten::run_script_string("get_input()"))
  }
}

#[cfg(not(target_os = "emscripten"))]
pub fn read_input() -> Option<String> {
  let mut input = String::new();

  match io::stdin().read_line(&mut input) {
    Ok(_) => { Some(String::from(input.trim())) },
    Err(error) => {
      println!("Error reading input: {}", error);
      None
    },
  }
}

#[cfg(target_os = "emscripten")]
pub fn sleep(ms: u64) {
  let script = format!("sleep({});", ms);
  emscripten::run_script(script.as_str());
}

#[cfg(not(target_os = "emscripten"))]
pub fn sleep(ms: u64) {
  let dur = ::std::time::Duration::from_millis(ms);
  ::std::thread::sleep(dur);
}

#[cfg(not(target_os = "emscripten"))]
pub fn random() -> f32 {
  rand::random::<f32>()
}

#[cfg(target_os = "emscripten")]
pub fn random() -> f32 {
  emscripten::random()
}

pub fn random_i32(min: i32, max: i32) -> i32 {
  let range = max - min;

  min + (random() * range as f32) as i32
}

#[cfg(not(target_os = "emscripten"))]
pub fn is_browser() -> bool { false }

#[cfg(target_os = "emscripten")]
pub fn is_browser() -> bool { true }

#[cfg(not(target_os = "emscripten"))]
pub fn clear_screen() {
  // Clear the screen.
  print!("{}[2J", 27 as char);

  // Move the cursor to the home position.
  print!("{}[H", 27 as char);
}

#[cfg(target_os = "emscripten")]
pub fn clear_screen() {
  emscripten::run_script("clear_screen()");
}

#[cfg(not(target_os = "emscripten"))]
pub fn writeln_with_wrapping(s: &str) {
  word_wrap::writeln_with_wrapping(s)
}

#[cfg(target_os = "emscripten")]
pub fn writeln_with_wrapping(s: &str) {
  // The browser will take care of line-wrapping for us.
  println!("{}", s)
}
