extern crate ww;

use std::io::{self, Write};
use std::ascii::AsciiExt;

use ww::map::{Map, MapRoomId};
use ww::room::Room;
use ww::direction::Direction;
use ww::direction::Direction::*;

use RoomId::*;

const MAX_ROOMS: usize = 20;

#[derive(Debug, Copy, Clone)]
enum RoomId {
  Hallway = 1,
  AudienceChamber = 2,
}

// Ideally we'd actually just get rid of MapRoomId and add a constraint
// to Map<T> requiring that T be type-castable as usize, but I don't
// know how to do that, so...
impl MapRoomId for RoomId {
  fn room_id(self) -> usize { self as usize }
}

#[derive(Debug)]
enum Command {
  Go(Direction),
  Look,
  Quit,
}

fn get_command() -> Command {
  loop {
    let mut input = String::new();

    io::stdout().write(b"> ").unwrap();
    io::stdout().flush().unwrap();

    match io::stdin().read_line(&mut input) {
      Ok(_) => {
        match input.chars().next() {
          Some(k) => {
            match k.to_ascii_lowercase() {
              'q' => { return Command::Quit; },
              'n' => { return Command::Go(North); },
              's' => { return Command::Go(South); },
              'e' => { return Command::Go(East); },
              'w' => { return Command::Go(West); },
              'l' => { return Command::Look; },
              _ => {},
            }
          },
          None => {}
        }
        println!("I have no idea what you're talking about.");
      },
      Err(error) => {
        println!("Error {}", error);
      }
    }
  }
}

fn main() {
  let mut rooms = [Room::new(); MAX_ROOMS];
  let mut map = Map::new(&mut rooms);

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

  map.connect(Hallway, South, AudienceChamber);

  let mut curr_room = Hallway;
  let mut show_desc = true;

  loop {
    if show_desc {
      println!("{}", map.room(curr_room).description);
      show_desc = false;
    }

    match get_command() {
      Command::Go(dir) => {
        if let Some(room) = map.room(curr_room).get_exit(dir) {
          curr_room = room;
          show_desc = true;
        } else {
          println!("You can't go that way.");
        }
      },
      Command::Look => { show_desc = true; }
      Command::Quit => { break; }
    }
  }

  println!("Farewell.");
}
