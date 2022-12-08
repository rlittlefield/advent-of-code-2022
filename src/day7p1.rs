#[path = "reader.rs"]
mod reader;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::BufReader;
use std::io::Read;

extern crate itertools;
use itertools::Itertools;
use itertools::PutBack;

use core::cmp::Ord;
use itertools::put_back;
use std::cell::RefCell;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead, Lines};
use std::path::Path;

use std::rc::Rc;

#[derive(Debug)]
enum Command {
    Back,
    Forward(String),
    List,
}

fn parse_command(line: String) -> Command {
    // this needs to return an enum
    // println!("{}", line);

    if line.starts_with("$ cd") {
        let (_, _cd, value) = line.split(" ").collect_tuple().unwrap();
        match value {
            ".." => {
                return Command::Back;
            }
            val => {
                return Command::Forward(val.to_owned());
            }
        }
    } else if line == "$ ls" {
        return Command::List;
    }
    panic!("uh oh, this didn't look like a valid command");
}

pub fn handle_commands(
    lines: Rc<RefCell<PutBack<Lines<BufReader<File>>>>>,
    directory_name: String,
) -> (u64, Vec<(String, u64, Vec<(String, u64)>)>) {
    let mut accumulator: Vec<(String, u64, Vec<(String, u64)>)> = vec![];
    // we return the accumulator, but the own_size is basically added into the accumulator if we ever do an ls in here
    let mut own_size: u64 = 0;
    let mut files: Vec<(String, u64)> = vec![];
    loop {
        let line = {
            if let Some(Ok(line)) = lines.borrow_mut().next() {
                Some(line)
            } else {
                None
            }
        };
        if let Some(line) = line {
            match parse_command(line) {
                Command::Forward(dir) => {
                    println!("changing to {} directory", dir);
                    let (child_size, result) =
                        handle_commands(Rc::clone(&lines), format!("{}/{}", directory_name, dir));
                    let mut subdir_size = 0;
                    own_size += child_size; // omg, this should have been obvious
                    for (subdir, size, files) in result {
                        accumulator.push((subdir, size, files));
                    }
                    println!(
                        "adding {} bytes to directory {} from child {}",
                        subdir_size, directory_name, dir
                    );
                    own_size += subdir_size;
                }
                Command::Back => {
                    println!("moving back a directory from {}", directory_name);
                    break; // this breaks out of the main loop, which returns the accumulator and everything so we don't need to do it here
                           // accumulator.push((directory_name.clone(), own_size));
                           // return accumulator;
                }
                Command::List => {
                    // this needs to collect everything until a new line starts with a $ again
                    // println!("listed directory {}", directory_name);

                    loop {
                        let line = {
                            if let Some(Ok(line)) = lines.borrow_mut().next() {
                                Some(line)
                            } else {
                                None
                            }
                        };

                        if let Some(line) = line {
                            if line.starts_with("$") {
                                // println!("putting back a command: {}", line);
                                // uhhh, put it back? This is an awful format and I hate it
                                {
                                    let mut borrowed_lines = lines.borrow_mut();
                                    borrowed_lines.put_back(Ok(line));
                                }
                                break;
                                // this line isn't a response, it is a new command. so we just put it back and break
                                // which will pop out of the List loop, and continue the outer line loop
                            }
                            if line.starts_with("dir") {
                                // println!("dir!");
                                continue;
                            }
                            // the only other thing apart from directories and new commands are actual files
                            let (size_str, filename) = line.split(" ").collect_tuple().unwrap();
                            let size = size_str.parse::<u64>().unwrap();
                            files.push((filename.to_owned(), size));
                            // let filename = filename.to_owned();
                            own_size += size;
                            println!(
                                "found file in directory {} with size: {}",
                                directory_name, size
                            );
                        } else {
                            // I guess this means the file is empty?
                            println!("file empty at end of list");
                            break;
                        }
                    }
                }
            }
        } else {
            break;
        }
    }
    println!(
        "finished with directory {} with size {}",
        directory_name, own_size
    );
    accumulator.push((directory_name.clone(), own_size, files));
    return (own_size, accumulator);
}

pub fn main() {
    if let Ok(mut lines) = reader::read_lines("./input7") {
        lines.next();
        let lines = put_back(lines);
        let lines = Rc::new(RefCell::new(lines));
        let (total_size, mut result) = handle_commands(lines, String::from(""));
        println!("total size: {}", total_size);
        // result.sort();
        print!("{:#?}", result);
        // let matching_items = result.into_iter().filter(|(_dir, size, _files)| *size <= 100000).collect_vec();
        // print!("{:#?}", matching_items);

        // let matching_sizes = matching_items.into_iter().filter(|(_dir, size, _files)| *size <= 100000).map(|(_dir, size, _files)| size).collect_vec();

        // println!("{:#?}", matching_sizes);
        // let sum: u64 = matching_sizes.into_iter().sum();
        // println!("sum of < 100000: {}", sum);

        // print!("{:#?}", result.into_iter().filter(|(_dir, size)| size.to_owned() < 100000).collect_vec());

        let disk_total = 70000000;
        let available_space = disk_total - total_size;
        let update_size = 30000000;
        let additional_space_required = update_size - available_space;

        let matching_sorted_dirs = result
            .into_iter()
            .filter(|(_, size, _)| size >= &additional_space_required)
            .sorted_by(|a, b| Ord::cmp(&b.1, &a.1))
            .collect_vec();
        println!("{:#?}", matching_sorted_dirs)
        //sorted_by(|_name, size, _files| size)

        // filter(|(_dir, size, _files)| *size <= 100000)
    }
}
