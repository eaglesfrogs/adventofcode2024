mod day1;
mod util;

use std::env;

fn main() {
    for argument in env::args() {
        match argument.as_str() {
            "day1" => day1::execute(),
            _ => println!("Day {argument} is not yet implemented"),
        }
    }
}
