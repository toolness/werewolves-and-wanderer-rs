use std::io::{self, Write};

const CHARS_PER_LINE: usize = 78;
const SPACE: u8 = 32;
const LF: u8 = 10;

// This is a *very* naive algorithm. It assumes the cursor is
// currently at column 0, and that the string to be printed is
// ASCII, among other things. It's good enough for our needs
// and much more lightweight than the current crates.io
// alternative, `textwrap`, which has a lot of dependencies.
pub fn writeln_with_wrapping(s: &str) {
  let stdout = io::stdout();
  let mut handle = stdout.lock();
  let bytes = s.as_bytes();
  let mut column = 0;
  let mut last_space = 0;
  let mut i = 0;

  for &c in bytes.iter() {
    i += 1;
    if c == SPACE || c == LF {
      handle.write(&bytes[last_space..i]).unwrap();
      if c == SPACE {
        column += i - last_space;
      } else {
        column = 0;
      }
      last_space = i;
    }
    if column + (i - last_space) >= CHARS_PER_LINE {
      if column == 0 {
        // Ack, we've got a really long word that exceeds the
        // length of a single line. Just write it out, breaking
        // it at the end of the line.
        handle.write(&bytes[last_space..i]).unwrap();
        last_space = i;
      }
      handle.write(b"\n").unwrap();
      column = 0;
    }
  }

  handle.write(&bytes[last_space..i]).unwrap();
  handle.write(b"\n").unwrap();
}
