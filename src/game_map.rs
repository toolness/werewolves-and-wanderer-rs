use platform::random_i32;
use direction::{Direction, NUM_DIRECTIONS};
use direction::Direction::*;
use monsters::MonsterId;
use sized_enum::SizedEnum;

use self::RoomId::*;
use self::RoomContents::*;

const NUM_ROOMS: usize = 19;
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

#[derive(Debug, Copy, Clone)]
pub enum RoomContents {
  Treasure(u8),
  Terror(MonsterId),
}

pub struct GameMap {
  rooms: Vec<Room>,
}

impl GameMap {
  pub fn new() -> Self {
    Self { rooms: vec![Room::new(); RoomId::size()] }
  }

  pub fn room(&self, r: RoomId) -> &Room {
    &self.rooms[r as usize]
  }

  pub fn mut_room(&mut self, r: RoomId) -> &mut Room {
    &mut self.rooms[r as usize]
  }

  pub fn connect(&mut self, from: RoomId, d: Direction,
                 to: RoomId) -> &mut Self {
    self.mut_room(from).set_exit(d, to);
    self.mut_room(to).set_exit(d.opposite(), from);
    self
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
        if room_id != Entrance && room_id != Exit &&
           room_id != Lift {
          let room = self.mut_room(room_id);
          if room.contents.is_none() {
            let contents = allotter();
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
      self.mut_room(room_id).contents = Some(Treasure(amount));
    }
  }

  fn describe_and_connect(&mut self) {
    self.mut_room(Entrance).describe(
      "Entrance",
      "You are at the entrance to a forbidding-looking \
       stone castle. You are facing east."
    );

    self.mut_room(Hallway).describe(
      "Hallway",
      "You are in the hallway. \
       There is a door to the south. \
       Through the windows to the north you can see a secret herb garden."
    );

    self.mut_room(AudienceChamber).describe(
      "Audience Chamber",
      "This is the audience chamber. \
       There is a window to the west. By looking to the right \
       through it you can see the entrance to the castle. \
       Doors leave this room to the north, east, and south."
    );

    self.mut_room(GreatHall).describe(
      "Great Hall",
      "You are in the great hall, an L-shaped room. \
       There are doors to the east and to the north. \
       In the alcove is a door to the west."
    );

    self.mut_room(PrivateMeeting).describe(
      "Private Meeting Room",
      "This is the monarch's private meeting room. \
       There is a single exit to the south."
    );

    self.mut_room(InnerHallway).describe(
      "Inner Hallway",
      "This inner hallway contains a door to the north, \
       and one to the west, and a circular stairwell \
       passes through the room. \
       You can see an ornamental lake through the \
       windows to the south."
    );

    self.mut_room(Kitchen).describe(
      "Kitchen",
      "This is the castle's kitchen. Through windows in \
       the north wall you can see a secret herb garden. \
       A door leaves the kitchen to the south."
    );

    self.mut_room(StoreRoom).describe(
      "Store Room",
      "You are in the store room, amidst spices, \
       vegetables, and vast sacks of flour and \
       other provisions. There is a door to the north \
       and one to the south."
    );

    self.mut_room(RearVestibule).describe(
      "Rear Vestibule",
      "You are in the rear vestibule. \
       There are windows to the south from which \
       you can see the ornamental lake. \
       There is an exit to the east, and \
       one to the north."
    );

    self.mut_room(Dungeon).describe(
      "Dungeon",
      "You are in the dank, dark dungeon. \
       There is a single exit, a small hole in \
       the wall towards the west."
    );

    self.mut_room(Guardroom).describe(
      "Guardroom",
      "You are in the prison guardroom, in the \
       basement of the castle. The stairwell \
       ends in this room. There is one other \
       exit, a small hole in the east wall."
    );

    self.mut_room(MasterBedroom).describe(
      "Master Bedroom",
      "You are in the master bedroom on the upper \
       level of the castle.... \
       Looking down from the window to the west you \
       can see the entrance to the castle, while the \
       secret herb garden is visible below the north \
       window. There are doors to the east and \
       to the south...."
    );

    self.mut_room(UpperHallway).describe(
      "Upper Hallway",
      "This is the L-shaped upper hallway. \
       To the north is a door, and there is a \
       stairwell in the hall as well. You can see \
       the lake through the south windows."
    );

    self.mut_room(Treasury).describe(
      "Treasury",
      "This room was used as the castle treasury in \
       by-gone years.... \
       There are no windows, just exits to the \
       north and to the east."
    );

    self.mut_room(ChambermaidsBedroom).describe(
      "Chambermaids' Bedroom",
      "Ooooh.... You are in the chambermaids' bedroom. \
       There is an exit to the west and a door \
       to the south...."
      // There's also a door to the north, but the book's
      // original description of this room doesn't include it.
      // Not sure if this means that it's meant to be a secret
      // or just a copy error.
    );

    self.mut_room(DressingChamber).describe(
      "Dressing Chamber",
      "This tiny room on the upper level is the \
       dressing chamber. There is a window to the \
       north, with a view of the herb garden down \
       below. A door leaves to the south."
    );

    self.mut_room(SmallRoom).describe(
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

#[derive(Debug, Copy, Clone)]
pub struct Room {
  exits: [Option<RoomId>; NUM_DIRECTIONS],
  pub name: &'static str,
  pub description: &'static str,
  pub contents: Option<RoomContents>,
}

impl Room {
  pub fn new() -> Self {
    Self {
      exits: [None; NUM_DIRECTIONS],
      name: "",
      description: "",
      contents: None,
    }
  }

  pub fn get_exit(self, d: Direction) -> Option<RoomId> {
    self.exits[d as usize]
  }

  pub fn set_exit(&mut self, d: Direction, r: RoomId) -> &mut Self {
    assert!(self.exits[d as usize].is_none());
    self.exits[d as usize] = Some(r);
    self
  }

  pub fn describe(&mut self, name: &'static str,
                  desc: &'static str) -> &mut Self {
    assert_eq!(self.name, "");
    self.name = name;
    self.description = desc;
    self
  }
}
