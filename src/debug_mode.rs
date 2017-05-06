use sized_enum::SizedEnum;
use enum_primitive::FromPrimitive;
use map::RoomId;
use game_state::{GameState, GameMode};

impl GameState {
  fn print_help(&self) {
    println!(
      "Debug mode commands:\n\n\

       quit            - exit debug mode\n\
       goto <room id>  - teleport to a room\n\
       rooms           - list rooms + their contents\n"
    );
  }

  fn goto_room(&mut self, args: Vec<&str>) {
    if let Ok(number) = args[0].parse::<usize>() {
      if let Some(room_id) = RoomId::from_usize(number) {
        self.curr_room = room_id;
        println!("Teleported to {:?}.", room_id);
        return;
      }
    }
    println!("Unknown room id.");
  }

  fn list_rooms(&self) {
    for i in 0..RoomId::size() {
      RoomId::from_usize(i).map(|room_id| {
        let room = self.map.room(room_id);
        println!("Room {} - {:?}", i, room_id);
        if let Some(contents) = room.contents {
          println!("  Contains {:?}", contents);
        }
      });
    }
  }

  pub fn tick_debug_mode(&mut self) {
    self.ask("debug> ", |state, input| {
      if input == "q" || input == "quit" {
        state.set_mode(GameMode::Primary);
      } else if input == "h" || input == "?" || input == "help" {
        state.print_help();
      } else if input == "rooms" {
        state.list_rooms();
      } else if input.starts_with("goto ") {
        state.goto_room(input.split_whitespace().skip(1).collect());
      } else if input.len() > 0 {
        println!("Unrecognized command. Type ? for help.");
      }
    });
  }
}
