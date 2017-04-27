#[cfg(target_os = "emscripten")]
pub mod emscripten;

#[cfg(not(target_os = "emscripten"))]
use std::io::{self, Write};

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
    Ok(_) => { return Some(input); },
    Err(error) => {
      println!("Error reading input: {}", error);
      return None;
    },
  }
}
