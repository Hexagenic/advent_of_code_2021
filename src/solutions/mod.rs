mod day1;

type PuzzleSolution = (Option<fn() -> i64>, Option<fn() -> i64>);

pub fn get_solution(day: u8) -> PuzzleSolution {
    match day {
        1 => (Some(day1::part1), Some(day1::part2)),
        _ => (None, None),
    }
}
