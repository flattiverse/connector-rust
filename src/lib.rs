#![allow(dead_code)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::excessive_precision)]
#![allow(clippy::new_ret_no_self)]
#![allow(clippy::useless_let_if_seq)]

extern crate byteorder;
extern crate sha2;
extern crate chrono;
extern crate hostname;
extern crate flate2;
extern crate rand;
extern crate atomic;
pub extern crate backtrace;

pub mod net;
pub mod unit;
pub mod item;
pub mod event;
pub mod dotnet;
pub mod message;
pub mod controllable;


mod error;
mod version;
mod connector;
mod difficulty;
mod polynominal;

mod task;
mod vector;

mod index_list;
mod block_manager;
mod universe_group;

mod color;
mod scores;
mod player;
mod platform_kind;
mod performance_test;
mod performance_mark;
mod performance_requirement;

mod team;
mod universe;
mod game_type;
mod stop_watch;
mod managed_array;
mod universal_holder;
mod universe_group_flow_control;

mod tournament;
mod tournament_team;





pub use crate::dotnet::*;


pub use crate::error::*;
pub use crate::version::*;
pub use crate::connector::*;
pub use crate::difficulty::*;
pub use crate::polynominal::*;

pub use crate::task::*;
pub use crate::vector::*;

pub use crate::color::*;
pub use crate::index_list::*;
pub use crate::block_manager::*;
pub use crate::managed_array::*;

pub use crate::scores::*;
pub use crate::message::*;
pub use crate::player::*;
pub use crate::platform_kind::*;
pub use crate::performance_test::*;
pub use crate::performance_mark::*;
pub use crate::performance_requirement::*;

pub use crate::team::*;
pub use crate::universe::*;
pub use crate::game_type::*;
pub use crate::stop_watch::*;
pub use crate::universe_group::*;
pub use crate::universal_holder::*;
pub use crate::universe_group_flow_control::*;

pub use crate::tournament::*;
pub use crate::tournament_team::*;


