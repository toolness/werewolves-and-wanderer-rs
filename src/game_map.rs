use enum_primitive::FromPrimitive;

use map;
use platform::random_i32;
use direction::Direction::*;

use self::RoomId::*;
use self::RoomContents::*;

const DEBUG: bool = false;
pub const NUM_ROOMS: usize = 19;
const NUM_MONSTERS: usize = 4;
const NUM_ROOMS_WITH_TREASURE: usize = 4;
const NUM_ROOMS_WITH_TERROR: usize = 4;
const MIN_TREASURE_AMOUNT: u8 = 10;
const MAX_TREASURE_AMOUNT: u8 = 110;

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

enum_from_primitive! {
#[derive(Debug, Copy, Clone)]
pub enum MonsterId {
  Werewolf = 0,
  Fleshgorger = 1,
  Maldemer = 2,
  Dragon = 3,
}
}

#[derive(Debug, Copy, Clone)]
pub enum RoomContents {
  Treasure(u8),
  Terror(MonsterId),
}

pub type GameMap<'a> = map::Map<'a, RoomId, RoomContents>;

impl<'a> GameMap<'a> {
  fn allot_terror(&mut self) {
    for _ in 0..NUM_ROOMS_WITH_TERROR {
      loop {
        let room_id = random_room_id();
        if room_id != Entrance && room_id != Exit {
          let room = self.room(room_id);
          if room.contents.is_none() {
            let monster_id = random_monster_id();
            if DEBUG {
              println!("DEBUG: Placing {:?} in {:?}.", monster_id, room_id);
            }
            room.contents = Some(Terror(monster_id));
            break;
          }
        }
      }
    }
  }

  fn allot_treasure(&mut self) {
    for _ in 0..NUM_ROOMS_WITH_TREASURE {
      loop {
        let room_id = random_room_id();
        if room_id != Entrance && room_id != Exit {
          let room = self.room(room_id);
          if room.contents.is_none() {
            let amount = random_treasure_amount();
            if DEBUG {
              println!("DEBUG: Placing ${} in {:?}.", amount, room_id);
            }
            room.contents = Some(Treasure(amount));
            break;
          }
        }
      }
    }
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

    self.connect(Entrance, East, Hallway);
    self.connect(Hallway, South, AudienceChamber);
  }
}

fn random_room_id() -> RoomId {
  loop {
    let r = random_i32(0, NUM_ROOMS as i32);
    match RoomId::from_i32(r) {
      Some(room_id) => { return room_id; },
      None => {}
    }
  }
}

fn random_monster_id() -> MonsterId {
  loop {
    let r = random_i32(0, NUM_MONSTERS as i32);
    match MonsterId::from_i32(r) {
      Some(monster_id) => { return monster_id; },
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

pub fn init(mut map: &mut GameMap) {
  map.describe_and_connect();
  map.allot_treasure();
  map.allot_terror();
  map.ensure_treasure();
}
