extern crate ww;

use ww::game_map::GameMap;
use ww::game_state::GameState;

fn main() {
  let mut rooms = GameMap::create_rooms();
  let mut map = GameMap::new(&mut rooms);

  map.populate();

  let mut state = GameState::new(&mut map);

  #[cfg(target_os = "emscripten")]
  ::ww::platform::emscripten::set_main_loop_callback(|| state.tick());

  #[cfg(not(target_os = "emscripten"))]
  while !state.is_finished() {
    state.tick();
  }

  println!("Farewell.");
}
