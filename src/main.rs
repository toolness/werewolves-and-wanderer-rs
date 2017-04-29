extern crate ww;

use ww::platform;
use ww::game_map::GameMap;
use ww::game_state::GameState;

fn main() {
  #[cfg(target_os = "windows")]
  platform::windows::enable_ansi();

  let mut rooms = GameMap::create_rooms();
  let mut map = GameMap::new(&mut rooms);

  map.populate();

  let mut state = GameState::new(&mut map);

  platform::clear_screen();

  #[cfg(target_os = "emscripten")]
  platform::emscripten::set_main_loop_callback(|| {
    if state.is_finished() {
      platform::emscripten::terminate_program();
    } else {
      state.tick()
    }
  });

  #[cfg(not(target_os = "emscripten"))]
  while !state.is_finished() {
    state.tick();
  }
}
