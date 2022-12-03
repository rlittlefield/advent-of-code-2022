use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[path = "reader.rs"] mod reader;

#[derive(Clone)]
enum Move {
    Rock,
    Paper,
    Scissors
}
#[derive(Clone)]
enum Strategy {
    Win,
    Lose,
    Tie
}

fn select_winning_move(them_move: Move) -> Move {
    match them_move {
        Move::Rock => { Move::Paper },
        Move::Paper => { Move:: Scissors },
        Move::Scissors => { Move::Rock },
    }
}


fn parse_them_move(selected_move: &str) -> Move {
    match selected_move {
        "A" => Move::Rock,
        "B" => Move::Paper,
        "C" => Move::Scissors,
        _ => panic!("oh fuck, this isn't a valid them move! {}", selected_move)
    }

}

// // this was only used for the first round
// fn parse_me_move(selected_move: &str) -> Move {
//     match selected_move {
//         "X" => Move::Rock,
//         "Y" => Move::Paper,
//         "Z" => Move::Scissors,
//         _ => panic!("oh fuck, this isn't a valid me move! {}", selected_move)
//     }

// }


fn parse_me_move(selected_move: &str) -> Strategy {
    match selected_move {
        "X" => Strategy::Lose,
        "Y" => Strategy::Tie,
        "Z" => Strategy::Win,
        _ => panic!("oh fuck, this isn't a valid me move! {}", selected_move)
    }

}


fn score_round(them: Move, strategy: Strategy) -> i32 {
    let mut round_score = 0;
    
    let me_move = match strategy {
        Strategy::Win => { select_winning_move(them.clone())},
        Strategy::Lose => { select_winning_move(select_winning_move(them.clone())) }, // this is gross, but :shrug:
        Strategy::Tie => { them.clone() }, // ties are easy
    };

    match me_move {
        Move::Rock => { round_score += 1},
        Move::Paper => { round_score += 2},
        Move::Scissors => { round_score += 3},
    }
    match (them, me_move) {
        // me winning
        (Move::Rock, Move::Paper) => { round_score += 6},
        (Move::Scissors, Move::Rock) => { round_score += 6},
        (Move::Paper, Move::Scissors) => { round_score += 6},
        // ties
        (Move::Rock, Move::Rock) => { round_score += 3},
        (Move::Paper, Move::Paper) => { round_score += 3},
        (Move::Scissors, Move::Scissors) => { round_score += 3},
        
        // me losing
        _ => {}
    }
    return round_score;
}


pub fn main() {
    if let Ok(lines) = reader::read_lines("./input2") {
        let mut total_score = 0;

        for line in lines {
            if let Ok(game_string) = line {
                let (them, me) = game_string.split_once(" ").expect("these should have been a valid move with a space");
                let round_score = score_round(parse_them_move(them), parse_me_move(me));
                total_score += round_score;
            }
        }
    
        print!("Day 1 total score {}", total_score);
    }
}



// fn main1() {
//     if let Ok(lines) = read_lines("./input") {
//         let mut elf_collector = vec![];
//         let mut current_elf: i32 = 0;

//         for line in lines {
//             if let Ok(number_string) = line {
//                 // if the contents of this line are a number, add it to the current item
//                 match number_string.parse::<i32>() {
//                     Ok(calories) => {
//                         current_elf += calories;
//                     },
//                     Err(_) => {
//                         elf_collector.push(current_elf);
//                         current_elf = 0;
//                     }
//                 }
//             }
//         }
//         elf_collector.sort_by(|a, b| b.cmp(a));
//         let top_3: i32 = elf_collector[0..3].to_vec().iter().sum();


//         print!("biggest 3 elves {}", top_3);
//     }
// }
