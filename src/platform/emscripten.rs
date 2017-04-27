// This file is based on:
// https://github.com/Gigoteur/PX8/blob/master/src/px8/emscripten.rs

use std::ffi::{CString, CStr};
use std::cell::RefCell;
use std::ptr::null_mut;
use std::os::raw::{c_int, c_void, c_float, c_char};

pub const FPS: c_int = 15;

#[allow(non_camel_case_types)]
type em_callback_func = unsafe extern fn();

extern {
    pub fn emscripten_set_main_loop(func: em_callback_func, fps: c_int, simulate_infinite_loop: c_int);
    pub fn emscripten_cancel_main_loop();
    pub fn emscripten_get_now() -> c_float;
    pub fn emscripten_random() -> c_float;
    pub fn emscripten_run_script(script: *const c_char);
    pub fn emscripten_run_script_int(script: *const c_char) -> c_int;
    pub fn emscripten_run_script_string(script: *const c_char) -> *const c_char;
}

thread_local!(static MAIN_LOOP_CALLBACK: RefCell<*mut c_void> = RefCell::new(null_mut()));

pub fn random() -> f32 {
  unsafe {
    emscripten_random()
  }
}

pub fn run_script(script: &str) {
  unsafe {
    emscripten_run_script(CString::new(script).unwrap().as_ptr())
  }
}

pub fn run_script_int(script: &str) -> c_int {
  unsafe {
    emscripten_run_script_int(CString::new(script).unwrap().as_ptr())
  }
}

pub fn run_script_string(script: &str) -> String {
  unsafe {
    let r = emscripten_run_script_string(CString::new(script).unwrap().as_ptr());
    let s = CStr::from_ptr(r);
    let our_string = String::from(s.to_string_lossy());
    return our_string;
  }
}

pub fn set_main_loop_callback<F>(callback: F) where F: FnMut() {
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
