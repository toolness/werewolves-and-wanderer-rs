#[macro_use] extern crate enum_primitive;

extern crate ww;

use enum_primitive::FromPrimitive;

use ww::map::{Map, MapRoomId};
use ww::room::Room;
use ww::platform;
use ww::direction::Direction::*;
use ww::command::{PrimaryCommand, CommandProcessor};

#[cfg(target_os = "emscripten")]
use ww::platform::emscripten;

use RoomId::*;
use RoomContents::*;

const DEBUG: bool = false;
const NUM_ROOMS: usize = 19;
const NUM_MONSTERS: usize = 4;
const NUM_ROOMS_WITH_TREASURE: usize = 4;
const NUM_ROOMS_WITH_TERROR: usize = 4;
const MIN_TREASURE_AMOUNT: u8 = 10;
const MAX_TREASURE_AMOUNT: u8 = 110;

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
enum RoomId {
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
enum MonsterId {
  Werewolf = 0,
  Fleshgorger = 1,
  Maldemer = 2,
  Dragon = 3,
}
}

#[derive(Debug, Copy, Clone)]
enum RoomContents {
  Treasure(u8),
  Terror(MonsterId),
}

type GameMap<'a> = Map<'a, RoomId, RoomContents>;

fn random_i32(min: i32, max: i32) -> i32 {
  let range = max - min;

  min + (platform::random() * range as f32) as i32
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

fn allot_terror(map: &mut GameMap) {
  for _ in 0..NUM_ROOMS_WITH_TERROR {
    loop {
      let room_id = random_room_id();
      if room_id != Entrance && room_id != Exit {
        let room = map.room(room_id);
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

fn allot_treasure(map: &mut GameMap) {
  for _ in 0..NUM_ROOMS_WITH_TREASURE {
    loop {
      let room_id = random_room_id();
      if room_id != Entrance && room_id != Exit {
        let room = map.room(room_id);
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

fn ensure_treasure(map: &mut GameMap) {
  for &room_id in [Treasury, PrivateMeeting].iter() {
    let amount = random_treasure_amount();
    if DEBUG {
      println!("DEBUG: Placing ${} in {:?}.", amount, room_id);
    }
    map.room(room_id).contents = Some(Treasure(amount));
  }
}

fn build_world(map: &mut GameMap) {
  map.room(Entrance).describe(
    "Entrance",
    "You are at the entrance to a forbidding-looking \
     stone castle. You are facing east."
  );

  map.room(Hallway).describe(
    "Hallway",
    "You are in the hallway. \
     There is a door to the south. \
     Through the windows to the north you can see a secret herb garden."
  );

  map.room(AudienceChamber).describe(
    "Audience Chamber",
    "This is the audience chamber. \
     There is a window to the west. By looking to the right \
     through it you can see the entrance to the castle. \
     Doors leave this room to the north, east, and south."
  );

  map.connect(Entrance, East, Hallway);
  map.connect(Hallway, South, AudienceChamber);
}

// Ideally we'd actually just get rid of MapRoomId and add a constraint
// to Map<T> requiring that T be type-castable as usize, but I don't
// know how to do that, so...
impl MapRoomId for RoomId {
  fn room_id(self) -> usize { self as usize }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum GameMode {
  AskName,
  Primary,
  Finished,
}

struct GameState<'a> {
  map: &'a mut GameMap<'a>,
  pub curr_mode: GameMode,
  player_name: String,
  curr_room: RoomId,
  show_desc: bool,
}

impl<'a> GameState<'a> {
  pub fn new(map: &'a mut GameMap<'a>) -> Self {
    Self {
      map: map,
      player_name: String::from(""),
      curr_mode: GameMode::AskName,
      curr_room: Entrance,
      show_desc: true,
    }
  }

  pub fn tick_primary_mode(&mut self) {
    if self.show_desc {
      println!("{}", self.map.room(self.curr_room).description);
      self.show_desc = false;
    }

    if let Some(cmd) = PrimaryCommand::get() {
      match cmd {
        PrimaryCommand::Go(dir) => {
          if let Some(room) = self.map.room(self.curr_room).get_exit(dir) {
            self.curr_room = room;
            self.show_desc = true;
          } else {
            println!("You can't go that way.");
          }
        },
        PrimaryCommand::Look => { self.show_desc = true; }
        PrimaryCommand::Quit => { self.curr_mode = GameMode::Finished; }
      }
    };
  }

  pub fn tick(&mut self) {
    match self.curr_mode {
      GameMode::AskName => {
        platform::show_prompt("What is your name, explorer? ");

        platform::read_input().map(|input| {
          if input.len() == 0 {
            println!("Pardon me?");
          } else {
            self.player_name = input;
            self.curr_mode = GameMode::Primary;
          }
        });
      },
      GameMode::Primary => { self.tick_primary_mode() },
      GameMode::Finished => {}
    }
  }
}

fn main() {
  let mut rooms = [Room::new(); NUM_ROOMS];
  let mut map = Map::new(&mut rooms);

  build_world(&mut map);
  allot_treasure(&mut map);
  allot_terror(&mut map);
  ensure_treasure(&mut map);

  let mut state = GameState::new(&mut map);

  println!("Werewolves and Wanderer\n");

  ::ww::platform::sleep(500);

  #[cfg(target_os = "emscripten")]
  emscripten::set_main_loop_callback(|| state.tick());

  #[cfg(not(target_os = "emscripten"))]
  while state.curr_mode != GameMode::Finished {
    state.tick();
  }

  println!("Farewell.");
}
