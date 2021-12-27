mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

#[derive(Debug, PartialEq, Eq)]
pub enum Solution {
    Integer(i64),
    String(String),
}

pub type PuzzleSolution = Option<fn(&str) -> Solution>;

#[must_use]
pub fn get_solution(day: u8) -> (PuzzleSolution, PuzzleSolution) {
    match day {
        1 => (Some(day1::part_a), Some(day1::part_b)),
        2 => (Some(day2::part_a), Some(day2::part_b)),
        3 => (Some(day3::part_a), Some(day3::part_b)),
        4 => (Some(day4::part_a), Some(day4::part_b)),
        5 => (Some(day5::part_a), Some(day5::part_b)),
        6 => (Some(day6::part_a), Some(day6::part_b)),
        7 => (Some(day7::part_a), Some(day7::part_b)),
        8 => (Some(day8::part_a), Some(day8::part_b)),
        9 => (Some(day9::part_a), None),
        10 => (Some(day10::part_a), Some(day10::part_b)),
        11 => (Some(day11::part_a), Some(day11::part_b)),
        12 => (Some(day12::part_a), None),
        13 => (Some(day13::part_a), Some(day13::part_b)),
        14 => (Some(day14::part_a), Some(day14::part_b)),
        15 => (Some(day15::part_a), Some(day15::part_b)),
        16 => (Some(day16::part_a), Some(day16::part_b)),
        17 => (Some(day17::part_a), Some(day17::part_b)),
        _ => (None, None),
    }
}

#[allow(unused)]
pub const MAX_SOLVED_DAY: u8 = 17;
