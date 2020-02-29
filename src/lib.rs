#![allow(unused)]
#![deny(intra_doc_link_resolution_failure)]

#[macro_use]
pub mod macros;

pub mod com;
pub mod command;
pub mod connector;
pub mod crypt;
pub mod entity;
pub mod io;
pub mod packet;
pub mod players;
pub mod requesting;
pub mod requests;
pub mod state;

#[macro_use]
extern crate log;

#[macro_use]
extern crate num_derive;
extern crate num_traits;
