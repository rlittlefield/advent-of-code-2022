// pub mod day_1_and_2;
// pub mod day3;
// pub mod day5;
pub mod day6p1;

pub mod reader;

#[macro_use]
extern crate timeit;

fn main() {
    let timed_result = timeit_loops!(1, {
        day6p1::main();
    });
    print!("timed: {}", timed_result);
}