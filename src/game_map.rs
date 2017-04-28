use enum_primitive::FromPrimitive;

use map;
use room::Room;
use platform::random_i32;
use direction::Direction::*;

use self::RoomId::*;
use self::RoomContents::*;

const DEBUG: bool = false;
const NUM_ROOMS: usize = 19;
const NUM_MONSTERS: usize = 4;
const NUM_ROOMS_WITH_TREASURE: usize = 4;
const NUM_ROOMS_WITH_TERROR: usize = 4;
const MIN_TREASURE_AMOUNT: u8 = 10;
const MAX_TREASURE_AMOUNT: u8 = 110;

// There doesn't seem to be a convenient way to get the "size" or
// "range" of an enum's possible values, so we'll make a trait for
// that here.
//
// Ideally we could populate it automatically through a macro, but
// for now we'll just implement it manually for all our enums.
trait SizedEnum {
  fn size() -> usize;
}

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RoomId {
  Hallway = 0,
  AudienceChamber = 1,
  GreatHall = 2,
  PrivateMeeting = 3,
  InnerHallway = 4,
  Entrance = 5,
  Kitchen = 6,
  StoreRoom = 7,
  Lift = 8,
  RearVestibule = 9,
  Exit = 10,
  Dungeon = 11,
  Guardroom = 12,
  MasterBedroom = 13,
  UpperHallway = 14,
  Treasury = 15,
  ChambermaidsBedroom = 16,
  DressingChamber = 17,
  SmallRoom = 18,
}
}

impl SizedEnum for RoomId {
  fn size() -> usize { NUM_ROOMS }
}

enum_from_primitive! {
#[derive(Debug, Copy, Clone)]
pub enum MonsterId {
  Werewolf = 0,
  Fleshgorger = 1,
  Maldemer = 2,
  Dragon = 3,
}
}

impl SizedEnum for MonsterId {
  fn size() -> usize { NUM_MONSTERS }
}

#[derive(Debug, Copy, Clone)]
pub enum RoomContents {
  Treasure(u8),
  Terror(MonsterId),
}

pub type GameMap<'a> = map::Map<'a, RoomId, RoomContents>;

impl<'a> GameMap<'a> {
  pub fn create_rooms() -> [Room<RoomId, RoomContents>; NUM_ROOMS] {
    [Room::new(); NUM_ROOMS]
  }

  pub fn populate(&mut self) {
    self.describe_and_connect();
    self.allot_treasure();
    self.allot_terror();
    self.ensure_treasure();
  }

  fn allot<F>(&mut self, num_rooms: usize, allotter: F)
    where F: Fn() -> RoomContents
  {
    for _ in 0..num_rooms {
      loop {
        let room_id = random_enum::<RoomId>();
        if room_id != Entrance && room_id != Exit {
          let room = self.room(room_id);
          if room.contents.is_none() {
            let contents = allotter();
            if DEBUG {
              println!("DEBUG: Placing {:?} in {:?}.", contents, room_id);
            }
            room.contents = Some(contents);
            break;
          }
        }
      }
    }
  }

  fn allot_terror(&mut self) {
    self.allot(NUM_ROOMS_WITH_TERROR, || Terror(random_enum::<MonsterId>()))
  }

  fn allot_treasure(&mut self) {
    self.allot(NUM_ROOMS_WITH_TREASURE,
               || Treasure(random_treasure_amount()))
  }

  fn ensure_treasure(&mut self) {
    for &room_id in [Treasury, PrivateMeeting].iter() {
      let amount = random_treasure_amount();
      if DEBUG {
        println!("DEBUG: Placing ${} in {:?}.", amount, room_id);
      }
      self.room(room_id).contents = Some(Treasure(amount));
    }
  }

  fn describe_and_connect(&mut self) {
    self.room(Entrance).describe(
      "Entrance",
      "You are at the entrance to a forbidding-looking \
       stone castle. You are facing east."
    );

    self.room(Hallway).describe(
      "Hallway",
      "You are in the hallway. \
       There is a door to the south. \
       Through the windows to the north you can see a secret herb garden."
    );

    self.room(AudienceChamber).describe(
      "Audience Chamber",
      "This is the audience chamber. \
       There is a window to the west. By looking to the right \
       through it you can see the entrance to the castle. \
       Doors leave this room to the north, east, and south."
    );

    self.room(GreatHall).describe(
      "Great Hall",
      "You are in the great hall, an L-shaped room. \
       There are doors to the east and to the north. \
       In the alcove is a door to the west."
    );

    self.room(PrivateMeeting).describe(
      "Private Meeting Room",
      "This is the monarch's private meeting room. \
       There is a single exit to the south."
    );

    self.room(InnerHallway).describe(
      "Inner Hallway",
      "This inner hallway contains a door to the north, \
       and one to the west, and a circular stairwell \
       passes through the room. \
       You can see an ornamental lake through the \
       windows to the south."
    );

    self.connect(Entrance, East, Hallway);
    self.connect(Hallway, South, AudienceChamber);
    self.connect(GreatHall, North, AudienceChamber);
    self.connect(GreatHall, West, AudienceChamber);
    self.connect(PrivateMeeting, South, InnerHallway);
    self.connect(InnerHallway, West, GreatHall);
    self.connect(InnerHallway, Up, UpperHallway);
  }
}

fn random_enum<T: FromPrimitive + SizedEnum>() -> T {
  loop {
    let r = random_i32(0, T::size() as i32);
    match T::from_i32(r) {
      Some(t) => { return t; },
      None => {}
    }
  }
}

fn random_treasure_amount() -> u8 {
  random_i32(MIN_TREASURE_AMOUNT as i32, MAX_TREASURE_AMOUNT as i32) as u8
}

// Ideally we'd actually just get rid of MapRoomId and add a constraint
// to Map<T> requiring that T be type-castable as usize, but I don't
// know how to do that, so...
impl map::MapRoomId for RoomId {
  fn room_id(self) -> usize { self as usize }
}
