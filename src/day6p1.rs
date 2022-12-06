
#[path = "reader.rs"] mod reader;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Read;
use std::collections::VecDeque;

extern crate itertools;
use itertools::Itertools;


use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::fs;



pub fn main() {
    let stack_count = 9;
    let mut started_commands = false;

    if let buf = reader::read_bytes("./input6") {
        let len = buf.len();
        for i in 0..len {
            let four_slice = &buf[i..i+14];
            let mut set = HashSet::new();
            for c in four_slice.into_iter() {
                set.insert(c.to_owned());
            }
            if set.len() == four_slice.len() {
                println!("looks like we found a match at index {}", i+14);
                println!("{:?}", set);
                println!("{:?}", four_slice);
                break;
            }

        }
    }
}
