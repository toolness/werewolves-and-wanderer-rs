use map;
use room::Room;
use platform::random_i32;
use direction::Direction::*;
use sized_enum::SizedEnum;

use self::RoomId::*;
use self::RoomContents::*;

const DEBUG: bool = false;
const NUM_ROOMS: usize = 19;
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
        let room_id = RoomId::random();
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
    self.allot(NUM_ROOMS_WITH_TERROR, || Terror(MonsterId::random()))
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

    self.room(Kitchen).describe(
      "Kitchen",
      "This is the castle's kitchen. Through windows in \
       the north wall you can see a secret herb garden. \
       A door leaves the kitchen to the south."
    );

    self.room(StoreRoom).describe(
      "Store Room",
      "You are in the store room, amidst spices, \
       vegetables, and vast sacks of flour and \
       other provisions. There is a door to the north \
       and one to the south."
    );

    self.room(RearVestibule).describe(
      "Rear Vestibule",
      "You are in the rear vestibule. \
       There are windows to the south from which \
       you can see the ornamental lake. \
       There is an exit to the east, and \
       one to the north."
    );

    self.room(Dungeon).describe(
      "Dungeon",
      "You are in the dank, dark dungeon. \
       There is a single exit, a small hole in \
       the wall towards the west."
    );

    self.room(Guardroom).describe(
      "Guardroom",
      "You are in the prison guardroom, in the \
       basement of the castle. The stairwell \
       ends in this room. There is one other \
       exit, a small hole in the east wall."
    );

    self.room(MasterBedroom).describe(
      "Master Bedroom",
      "You are in the master bedroom on the upper \
       level of the castle.... \
       Looking down from the window to the west you \
       can see the entrance to the castle, while the \
       secret herb garden is visible below the north \
       window. There are doors to the east and \
       to the south...."
    );

    self.room(UpperHallway).describe(
      "Upper Hallway",
      "This is the L-shaped upper hallway. \
       To the north is a door, and there is a \
       stairwell in the hall as well. You can see \
       the lake through the south windows."
    );

    self.room(Treasury).describe(
      "Treasury",
      "This room was used as the castle treasury in \
       by-gone years.... \
       There are no windows, just exits to the \
       north and to the east."
    );

    self.room(ChambermaidsBedroom).describe(
      "Chambermaids' Bedroom",
      "Ooooh.... You are in the chambermaids' bedroom. \
       There is an exit to the west and a door \
       to the south...."
      // There's also a door to the north, but the book's
      // original description of this room doesn't include it.
      // Not sure if this means that it's meant to be a secret
      // or just a copy error.
    );

    self.room(DressingChamber).describe(
      "Dressing Chamber",
      "This tiny room on the upper level is the \
       dressing chamber. There is a window to the \
       north, with a view of the herb garden down \
       below. A door leaves to the south."
    );

    self.room(SmallRoom).describe(
      "Small Room",
      "This is the small room outside the castle \
       lift which can be entered by a door to the north. \
       Another door leads to the west. You can see \
       the lake through the southern windows."
    );

    self.connect(Entrance, East, Hallway);
    self.connect(Hallway, South, AudienceChamber);
    self.connect(GreatHall, North, AudienceChamber);
    self.connect(GreatHall, West, AudienceChamber);
    self.connect(PrivateMeeting, South, InnerHallway);
    self.connect(InnerHallway, West, GreatHall);
    self.connect(InnerHallway, Up, UpperHallway);
    self.connect(Kitchen, South, StoreRoom);
    self.connect(StoreRoom, South, RearVestibule);
    self.connect(RearVestibule, East, Exit);
    self.connect(Dungeon, West, Guardroom);
    self.connect(Guardroom, Up, InnerHallway);
    self.connect(MasterBedroom, South, UpperHallway);
    self.connect(MasterBedroom, East, ChambermaidsBedroom);
    self.connect(Treasury, North, ChambermaidsBedroom);
    self.connect(Treasury, East, SmallRoom);
    self.connect(ChambermaidsBedroom, North, DressingChamber);
    self.connect(SmallRoom, North, Lift);
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
