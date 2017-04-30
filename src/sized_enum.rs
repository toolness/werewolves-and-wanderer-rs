use enum_primitive::FromPrimitive;

use platform::random_i32;

// There doesn't seem to be a convenient way to get the "size" or
// "range" of an enum's possible values, so we'll make a trait for
// that here.
//
// Ideally we could populate it automatically through a macro, but
// for now we'll just implement it manually for all our enums.
pub trait SizedEnum : FromPrimitive {
  fn size() -> usize;

  fn random() -> Self {
    loop {
      let r = random_i32(0, Self::size() as i32);
      match Self::from_i32(r) {
        Some(t) => { return t; },
        None => {}
      }
    }
  }
}
