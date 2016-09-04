#![feature(collections)]
#![no_std]


#[macro_use(format)]
extern crate collections;
extern crate goblin;


mod elf;


pub use elf::Elf;
