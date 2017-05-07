#[macro_use] extern crate enum_primitive;

#[macro_use] pub mod platform;
pub mod direction;
#[macro_use] pub mod command;
pub mod map;
pub mod game_state;
pub mod primary_mode;
pub mod combat;
pub mod items;
pub mod monsters;
pub mod inventory;
pub mod util;
pub mod sized_enum;

#[cfg(debug_assertions)]
pub mod debug_mode;
