
#[path = "reader.rs"] mod reader;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Read;

extern crate itertools;
use itertools::Itertools;


fn str_set(input: &str) -> HashSet<char> {
    let mut set = HashSet::new();
    for c in input.chars() {
        set.insert(c);
    }
    return set
}



fn byte_set(input: &Vec<u8>) -> HashSet<&u8> {
    let mut set = HashSet::new();
    for c in input {
        set.insert(c);
    }
    return set
}

fn get_byte_score(mychar: &u8) -> i32 {
    match mychar {
        b'a' => 1,
        b'b' => 2,
        b'c' => 3,
        b'd' => 4,
        b'e' => 5,
        b'f' => 6,
        b'g' => 7,
        b'h' => 8,
        b'i' => 9,
        b'j' => 10,
        b'k' => 11,
        b'l' => 12,
        b'm' => 13,
        b'n' => 14,
        b'o' => 15,
        b'p' => 16,
        b'q' => 17,
        b'r' => 18,
        b's' => 19,
        b't' => 20,
        b'u' => 21,
        b'v' => 22,
        b'w' => 23,
        b'x' => 24,
        b'y' => 25,
        b'z' => 26,
        b'A' => 27,
        b'B' => 28,
        b'C' => 29,
        b'D' => 30,
        b'E' => 31,
        b'F' => 32,
        b'G' => 33,
        b'H' => 34,
        b'I' => 35,
        b'J' => 36,
        b'K' => 37,
        b'L' => 38,
        b'M' => 39,
        b'N' => 40,
        b'O' => 41,
        b'P' => 42,
        b'Q' => 43,
        b'R' => 44,
        b'S' => 45,
        b'T' => 46,
        b'U' => 47,
        b'V' => 48,
        b'W' => 49,
        b'X' => 50,
        b'Y' => 51,
        b'Z' => 52,
        _ => panic!("This isn't a valid letter")
    }
}


fn get_char_score(mychar: &char) -> i32 {
    match mychar {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => panic!("This isn't a valid letter")
    }
}



pub fn main() {
    if let Ok(lines) = reader::read_byte_lines("./input3") {
        let mut running_total = 0;
        let groups =  lines.tuples();
        for (a, b, c) in groups {
            let a = a.unwrap();
            let b = b.unwrap();
            let c = c.unwrap();

            let a_set = byte_set(&a);
            let b_set = byte_set(&b);
            let c_set = byte_set(&c);

            let intersection = a_set.intersection(&b_set);
            let mut interset = HashSet::new();
            for c in intersection {
                interset.insert(c.clone());
            }
            let intersection2 = interset.intersection(&c_set);
            let item = intersection2.last().unwrap();

            // let index = alphabet.chars().position(|c| c == *item).unwrap() + 1;
            running_total += get_byte_score(item);

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
