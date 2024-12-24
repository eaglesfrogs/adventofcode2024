mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod util;

use std::env;

fn main() {
    for argument in env::args() {
        if argument.contains("adventofcode2024") {
            continue;
        }

        match argument.as_str() {
            "day01" => day01::execute(),
            "day02" => day02::execute(),
            "day03" => day03::execute(),
            "day04" => day04::execute(),
            "day05" => day05::execute(),
            "day06" => day06::execute(),
            "day07" => day07::execute(),
            "day08" => day08::execute(),
            "day09" => day09::execute(),
            "day10" => day10::execute(),
            "day11" => day11::execute(),
            "day12" => day12::execute(),
            "day13" => day13::execute(),
            "day14" => day14::execute(),
            "day15" => day15::execute(),
            "day16" => day16::execute(),
            "day17" => day17::execute(),
            "day18" => day18::execute(),
            "day19" => day19::execute(),
            "day20" => day20::execute(),
            "day21" => day21::execute(),
            "day22" => day22::execute(),
            "day23" => day23::execute(),
            "day24" => day24::execute(),
            _ => println!("Day {argument} is not yet implemented"),
        }
    }
}
