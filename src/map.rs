use room::Room;
use direction::Direction;

pub trait MapRoomId {
  fn room_id(self) -> usize;
}

#[derive(Debug)]
pub struct Map<'a, T: 'a + Copy + MapRoomId> {
  rooms: &'a mut [Room<T>]
}

impl<'a, T: Copy + MapRoomId> Map<'a, T> {
  pub fn new(rooms: &'a mut [Room<T>]) -> Self {
    Self { rooms: rooms }
  }

  pub fn room(&mut self, r: T) -> &mut Room<T> {
    &mut self.rooms[r.room_id()]
  }

  pub fn connect(&mut self, from: T, d: Direction, to: T) -> &mut Self {
    self.room(from).set_exit(d, to);
    self.room(to).set_exit(d.opposite(), from);
    self
  }
}
