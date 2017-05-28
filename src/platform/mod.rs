#[cfg(target_os = "emscripten")]
mod emscripten;

#[cfg(not(target_os = "emscripten"))]
mod stdio;

macro_rules! wrapln {
  ( ) => {{
    use ::platform::*;
    Platform::writeln_with_wrapping("");
  }};
  ( $fmt:expr ) => {{
    use ::platform::*;
    Platform::writeln_with_wrapping($fmt);
  }};
  ( $fmt:expr, $($arg:tt)* ) => {{
    use ::platform::*;
    Platform::writeln_with_wrapping(format!($fmt, $($arg)*));
  }}
}

pub trait AbstractPlatform {
  fn init() {
  }

  fn show_prompt(prompt: &str);

  fn hide_prompt() {
    Self::show_prompt("");
  }

  fn read_input() -> Option<String>;

  fn sleep(ms: u64);

  fn random() -> f32;

  fn random_i32(min: i32, max: i32) -> i32 {
    let range = max - min;

    min + (Self::random() * range as f32) as i32
  }

  fn is_browser() -> bool;

  fn clear_screen();

  fn writeln_with_wrapping<T: AsRef<str>>(s: T);

  fn terminate_program();

  fn set_main_loop_callback<F: FnMut()>(callback: F);
}

#[cfg(target_os = "emscripten")]
pub type Platform = emscripten::EmscriptenPlatform;

#[cfg(not(target_os = "emscripten"))]
pub type Platform = stdio::StdioPlatform;
