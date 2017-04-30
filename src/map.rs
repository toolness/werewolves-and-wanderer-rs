use room::Room;
use direction::Direction;

pub trait MapRoomId {
  fn room_id(self) -> usize;
}

#[derive(Debug)]
pub struct Map<'a, T: 'a + Copy + MapRoomId, C: 'a + Copy> {
  rooms: &'a mut [Room<T, C>]
}

impl<'a, T: Copy + MapRoomId, C: Copy> Map<'a, T, C> {
  pub fn new(rooms: &'a mut [Room<T, C>]) -> Self {
    Self { rooms: rooms }
  }

  pub fn room(&self, r: T) -> &Room<T, C> {
    &self.rooms[r.room_id()]
  }

  pub fn mut_room(&mut self, r: T) -> &mut Room<T, C> {
    &mut self.rooms[r.room_id()]
  }

  pub fn connect(&mut self, from: T, d: Direction, to: T) -> &mut Self {
    self.mut_room(from).set_exit(d, to);
    self.mut_room(to).set_exit(d.opposite(), from);
    self
  }
}
