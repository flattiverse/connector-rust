#![feature(const_fn)]
#![allow(dead_code)]

extern crate byteorder;
extern crate sha2;
extern crate chrono;
extern crate hostname;

#[macro_use]
extern crate downcast_rs;

mod net;
mod unit;
mod item;
mod event;
mod dotnet;
mod message;


mod error;
mod connector;
mod version;

mod task;
mod vector;

mod block;
mod index_list;
mod block_manager;

mod color;
mod scores;
mod player;
mod platform_kind;
mod performance_mark;

mod team;
mod game_type;
mod universe_group;
mod universal_holder;



pub use dotnet::*;


pub use error::*;
pub use connector::*;
pub use version::*;

pub use task::*;
pub use vector::*;

pub use color::*;
pub use block::*;
pub use index_list::*;
pub use block_manager::*;

pub use scores::*;
pub use message::*;
pub use player::*;
pub use platform_kind::*;
pub use performance_mark::*;

pub use team::*;
pub use game_type::*;
pub use universe_group::*;
pub use universal_holder::*;

pub use downcast_rs::Downcast;