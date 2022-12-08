use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// found this somewhere online in rust docs. lines() is nice
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// this is like the same as above, but we are trying to do bytes strings not unicodes
pub fn read_byte_lines<P>(filename: P) -> io::Result<io::Split<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).split(b'\n'))
}

pub fn read_bytes(filename: &str) -> Vec<u8> {
    fs::read(filename).unwrap()
}
