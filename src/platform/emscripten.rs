// A lot of this file is based on:
// https://github.com/Gigoteur/PX8/blob/master/src/px8/emscripten.rs

use std::ffi::{CString, CStr};
use std::cell::RefCell;
use std::ptr::null_mut;
use std::os::raw::{c_int, c_void, c_float, c_char};

use platform::AbstractPlatform;

const FPS: c_int = 1;

#[allow(non_camel_case_types)]
type em_callback_func = unsafe extern fn();

extern {
  fn emscripten_set_main_loop(func: em_callback_func, fps: c_int, simulate_infinite_loop: c_int);
  fn emscripten_cancel_main_loop();
  fn emscripten_random() -> c_float;
  fn emscripten_run_script(script: *const c_char);
  fn emscripten_run_script_int(script: *const c_char) -> c_int;
  fn emscripten_run_script_string(script: *const c_char) -> *const c_char;
}

thread_local!(static MAIN_LOOP_CALLBACK: RefCell<*mut c_void> = RefCell::new(null_mut()));

fn run_script(script: &str) {
  unsafe {
    emscripten_run_script(CString::new(script).unwrap().as_ptr())
  }
}

fn run_script_int(script: &str) -> c_int {
  unsafe {
    emscripten_run_script_int(CString::new(script).unwrap().as_ptr())
  }
}

fn run_script_string(script: &str) -> String {
  unsafe {
    let r = emscripten_run_script_string(CString::new(script).unwrap().as_ptr());
    let s = CStr::from_ptr(r);
    let our_string = String::from(s.to_string_lossy());
    return our_string;
  }
}

pub struct EmscriptenPlatform;

impl AbstractPlatform for EmscriptenPlatform {
  fn show_prompt(prompt: &str) {
    let script = format!("set_prompt({:?});", prompt);
    run_script(script.as_str());
  }

  fn read_input() -> Option<String> {
    if run_script_int("has_input()") == 0 {
      None
    } else {
      Some(run_script_string("get_input()"))
    }
  }

  fn sleep(ms: u64) {
    let script = format!("sleep({});", ms);
    run_script(script.as_str());
  }

  fn random() -> f32 {
    unsafe {
      emscripten_random()
    }
  }

  fn clear_screen() {
    run_script("clear_screen()");
  }

  fn writeln_with_wrapping<T: AsRef<str>>(s: T) {
    // The browser will take care of line-wrapping for us.
    println!("{}", s.as_ref())
  }

  fn terminate_program() {
    run_script("terminate_program()");
    unsafe { emscripten_cancel_main_loop() }
  }

  fn set_main_loop_callback<F>(callback: F) where F: FnMut() {
    MAIN_LOOP_CALLBACK.with(|log| {
      *log.borrow_mut() = &callback as *const _ as *mut c_void;
    });

    unsafe { emscripten_set_main_loop(wrapper::<F>, FPS, 1); }

    unsafe extern "C" fn wrapper<F>() where F: FnMut() {
      MAIN_LOOP_CALLBACK.with(|z| {
        let closure = *z.borrow_mut() as *mut F;
        (*closure)();
      });
    }
  }
}
