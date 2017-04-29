extern crate kernel32;
extern crate winapi;

// This was added in a Windows 10 update, so it'll only work on
// Windows 10.
const ENABLE_VIRTUAL_TERMINAL_PROCESSING: winapi::minwindef::DWORD = 0x0004;

pub fn enable_ansi() {
  unsafe {
    // http://stackoverflow.com/q/38772468/2422398
    let handle = kernel32::GetStdHandle(winapi::winbase::STD_OUTPUT_HANDLE);
    let mut dwmode: winapi::minwindef::DWORD = 0;
    kernel32::GetConsoleMode(handle, &mut dwmode);
    dwmode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;
    kernel32::SetConsoleMode(handle, dwmode);
  }
}
