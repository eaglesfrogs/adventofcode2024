mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod util;

use std::env;

fn main() {
    for argument in env::args() {
        match argument.as_str() {
            "day01" => day01::execute(),
            "day02" => day02::execute(),
            "day03" => day03::execute(),
            "day04" => day04::execute(),
            "day05" => day05::execute(),
            "day06" => day06::execute(),
            "day07" => day07::execute(),
            _ => println!("Day {argument} is not yet implemented"),
        }
    }
}
