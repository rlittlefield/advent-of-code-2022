
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

    if let Ok(lines) = reader::read_lines("./input5") {
        let mut stacks: VecDeque<VecDeque<char>> = VecDeque::new();
        for i in 0..stack_count {
            stacks.push_back(VecDeque::new())
        }

        for line in lines {
            match line{
                Ok(line) => {
                    match line.as_str() {
                        "" => {
                            started_commands = true;
                            println!("stacks: {:?}", stacks)
                        },
                        line => {
                            if !started_commands {
                                if line.starts_with(" 1") {
                                    continue; // this is the way to number each of the columns. Hopefully they don't change that
                                }
                                println!("{}", line);

                                // load all the initial state here
                                for i in 0..stack_count {
                                    let crate_content = line.chars().nth((i * 4) + 1).expect("crate label missing from input file");
                                    if crate_content != ' ' {
                                        stacks.get_mut(i).expect("should have been a stack at this index").push_back(crate_content);
                                    }
                                }
                            } else {
                                // this is the "move N from STACKA to STACKB"
                                let parts: Vec<&str>= line.split(" ").collect();
                                let quantity = parts[1].parse::<u32>().unwrap();
                                let source_index = parts[3].parse::<usize>().unwrap() - 1; // zero indexed stacks vec
                                let destination_index = parts[5].parse::<usize>().unwrap() -1;
                                
                                let mut extraction = VecDeque::new();
                                let source_stack = stacks.get_mut(source_index).unwrap();

                                for i in 0..quantity {
                                    extraction.push_front(source_stack.pop_front().expect("missing crate from source stack"));
                                }

                                // println!("{:?}", extraction);

                                let destination_stack = stacks.get_mut(destination_index).unwrap();


                                for i in 0..quantity {
                                    let item = extraction.pop_front().expect("if this fails, the instructions are wrong because one of the source stacks is empty");
                                    // println!("{}", item);
                                    destination_stack.push_front(item);
                                    // println!("{:?}", destination_stack);
                                }

                                // panic!("panic");

                            }      
                        }
                    }
                                  
                },
                _ => {}
            }
        }
        // println!("running total: {}", running_total);
        println!("stacks: {:?}", stacks)
    
        
    }
}
