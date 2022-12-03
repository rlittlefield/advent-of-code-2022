
#[path = "reader.rs"] mod reader;
use std::collections::HashMap;
use std::collections::HashSet;

extern crate itertools;
use itertools::Itertools;


fn str_set(input: &str) -> HashSet<char> {
    let mut set = HashSet::new();
    for c in input.chars() {
        set.insert(c);
    }
    return set
}




pub fn main() {
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";


    if let Ok(lines) = reader::read_lines("./input3") {
        let mut running_total = 0;
        let groups =  lines.tuples();
        for (a, b, c) in groups {
            let a = a.unwrap();
            let b = b.unwrap();
            let c = c.unwrap();

            let mut a_set = str_set(&a);
            let mut b_set = str_set(&b);
            let mut c_set = str_set(&c);

            let intersection = a_set.intersection(&b_set);
            let mut interset = HashSet::new();
            for c in intersection {
                interset.insert(c.clone());
            }
            let intersection2 = interset.intersection(&c_set);
            let item = intersection2.last().unwrap();

            let index = alphabet.chars().position(|c| c == *item).unwrap() + 1;
            running_total += index;

            // if let Ok(game_string) = line {
            //     let (left, right) = game_string.split_at(game_string.len() / 2);
            //     let mut left_set = str_set(left);
            //     let mut right_set = str_set(right);
            //     let intersection = left_set.intersection(&right_set);
            //     let item = intersection.last().unwrap(); // like a present!

            //     // replace this with a hashmap or something if we need to speed it up
            //     let index = alphabet.chars().position(|c| c == *item).unwrap() + 1;

            //     running_total += index;
            // }
        }
        println!("running total: {}", running_total);

    
        
    }
}
