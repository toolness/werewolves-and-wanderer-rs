use std::marker::PhantomData;
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

  fn iter() -> SizedEnumIterator<Self> {
    SizedEnumIterator::new()
  }

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

pub struct SizedEnumIterator<T: FromPrimitive> {
  current: i32,
  phantom: PhantomData<T>,
}

impl<T: FromPrimitive> SizedEnumIterator<T> {
  fn new() -> Self {
    Self { current: 0, phantom: PhantomData }
  }
}

impl<T: FromPrimitive> Iterator for SizedEnumIterator<T> {
  type Item = T;

  fn next(&mut self) -> Option<T> {
    T::from_i32(self.current)
  }
}
