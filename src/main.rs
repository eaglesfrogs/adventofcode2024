mod day01;
mod day02;
mod day03;
mod util;

use std::env;

fn main() {
    for argument in env::args() {
        match argument.as_str() {
            "day01" => day01::execute(),
            "day02" => day02::execute(),
            "day03" => day03::execute(),
            _ => println!("Day {argument} is not yet implemented"),
        }
    }
}
