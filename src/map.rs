use room::Room;
use direction::Direction;
use sized_enum::SizedEnum;

pub trait MapRoomId {
  fn room_id(self) -> usize;
}

#[derive(Debug)]
pub struct Map<T: Copy + MapRoomId + SizedEnum, C: Copy> {
  rooms: Vec<Room<T, C>>
}

impl<T: Copy + MapRoomId + SizedEnum, C: Copy> Map<T, C> {
  pub fn new() -> Self {
    Self { rooms: vec![Room::new(); T::size()] }
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
