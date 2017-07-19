#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(test, allow(dead_code))]

#![feature(test)]
#![allow(dead_code)]

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate lazy_static;

extern crate chashmap;
extern crate rand;

extern crate test;
pub mod board;
pub mod bit_twiddles;
pub mod movegen;
pub mod piece_move;
pub mod templates;
pub mod magic_helper;
pub mod transposition_table;



//include!("tests/test.rs");

