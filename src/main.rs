extern crate ww;

use std::cell::RefCell;

use ww::platform::*;
use ww::game_state::GameState;

thread_local!(static GAME_STATE: RefCell<GameState> = {
  let mut state = GameState::new();

  state.map.populate();

  RefCell::new(state)
});

fn main() {
  Platform::init();

  Platform::clear_screen();

  Platform::set_main_loop_callback(|| game_state_tick());
}
 
#[no_mangle]
pub extern fn game_state_tick() {
  GAME_STATE.with(|refcell| {
    let ref mut state = *refcell.borrow_mut();

    state.tick();
    while !state.is_waiting_for_input() {
      state.tick();
      if state.is_finished() {
        Platform::terminate_program();
        return;
      }
    }
  });
}
