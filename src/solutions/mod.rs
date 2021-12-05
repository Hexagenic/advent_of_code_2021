mod day1;
mod day2;
mod day3;

pub type PuzzleSolution = Option<fn(&str) -> i64>;

#[must_use]
pub fn get_solution(day: u8) -> (PuzzleSolution, PuzzleSolution) {
    match day {
        1 => (Some(day1::part_a), Some(day1::part_b)),
        2 => (Some(day2::part_a), Some(day2::part_b)),
        3 => (Some(day3::part_a), None),
        _ => (None, None),
    }
}

#[allow(unused)]
pub const MAX_SOLVED_DAY: u8 = 3;