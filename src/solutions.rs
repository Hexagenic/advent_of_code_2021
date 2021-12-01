use crate::error::Error;

type PuzzleSolution = fn() -> Result<(Option<i64>, Option<i64>), Error>;

#[must_use]
pub fn get_solution(day: u8) -> PuzzleSolution {
    match day {
        1 => day1,
        _ => || Ok((None, None)),
    }
}

#[allow(unused)]
pub const MAX_SOLVED_DAY: u8 = 1;

#[allow(clippy::missing_errors_doc)]
pub fn day1() -> Result<(Option<i64>, Option<i64>), Error> {
    let path = std::env::current_dir()?.join("input").join("day1.txt");

    let file = std::fs::read_to_string(path)?;
    let lines: Vec<i32> = file.split_whitespace().flat_map(str::parse).collect();

    let mut last_line: Option<i32> = None;

    let mut count_single = 0;
    let mut count_rolling = 0;

    for line in &lines {
        if let Some(last_line) = last_line {
            if *line > last_line {
                count_single += 1;
            }
        }
        last_line = Some(*line);
    }

    last_line = None;

    let a_it = lines.iter();
    let b_it = lines.iter().skip(1);
    let c_it = lines.iter().skip(2);
    let rolling_window = a_it.zip(b_it).zip(c_it);

    for ((a, b), c) in rolling_window {
        let line = a + b + c;
        if let Some(last_line) = last_line {
            if line > last_line {
                count_rolling += 1;
            }
        }
        last_line = Some(line);
    }

    Ok((Some(count_single), Some(count_rolling)))
}
