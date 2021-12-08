mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day8;

pub type PuzzleSolution = Option<fn(&str) -> i64>;

#[must_use]
pub fn get_solution(day: u8) -> (PuzzleSolution, PuzzleSolution) {
    match day {
        1 => (Some(day1::part_a), Some(day1::part_b)),
        2 => (Some(day2::part_a), Some(day2::part_b)),
        3 => (Some(day3::part_a), Some(day3::part_b)),
        4 => (Some(day4::part_a), Some(day4::part_b)),
        5 => (Some(day5::part_a), Some(day5::part_b)),
        6 => (Some(day6::part_a), Some(day6::part_b)),
        8 => (Some(day8::part_a), Some(day8::part_b)),
        _ => (None, None),
    }
}

#[allow(unused)]
pub const MAX_SOLVED_DAY: u8 = 8;
