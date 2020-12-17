#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate num_integer;

mod file;

mod day16;

fn main() {
    day16::main()
}
