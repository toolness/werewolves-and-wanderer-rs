pub mod direction;
pub mod map;
pub mod room;
pub mod command;

#[cfg(target_os = "emscripten")]
pub mod emscripten;
